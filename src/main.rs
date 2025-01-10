use std::{borrow::BorrowMut, collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::{Context, Error};
use clap::Parser;
use walrus::{
    ir::{Instr, InstrSeqId},
    passes::gc,
    ElementItems, ExportItem, FunctionId, FunctionKind, ImportKind, LocalFunction, Module,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pub file: PathBuf,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let s = StripSeq(vec![
        Arc::new(CallReplace(vec!["__shim_".to_string()])),
        Arc::new(StartEntry),
        Arc::new(StartExport),
        Arc::new(Unused),
    ]);

    // Load module
    let mut m = Module::from_file(&cli.file).context("failed to load module")?;

    // Strip Wasi
    s.strip(&mut m).context("failed to strip module")?;

    // Write module
    m.emit_wasm_file(cli.output.unwrap_or(cli.file))?;

    Ok(())
}

const PREFIX_INIT: &str = "_initialize";
const PREFIX_P1: &str = "wasi_snapshot_preview1";
const PREFIX_UNSTABLE: &str = "wasi_unstable";

trait Strip: Send + Sync {
    fn strip(&self, m: &mut Module) -> Result<(), Error>;
}

struct StripSeq(Vec<Arc<dyn Strip>>);

impl Strip for StripSeq {
    fn strip(&self, m: &mut Module) -> Result<(), Error> {
        for s in self.0.iter() {
            s.strip(m).context("failed to strip")?;
        }

        Ok(())
    }
}

struct CallReplace(
    Vec<String>, // Prefixes
);

impl Strip for CallReplace {
    fn strip(&self, m: &mut Module) -> Result<(), Error> {
        let imps: HashMap<String, FunctionId> = m
            .imports
            .iter()
            .filter(|i| [PREFIX_P1, PREFIX_UNSTABLE].contains(&i.module.as_ref()))
            .flat_map(|i| match i.kind {
                ImportKind::Function(id) => Some((i.name.to_owned(), id)),
                _ => None,
            })
            .collect();

        let fs: HashMap<String, FunctionId> = m
            .funcs
            .iter()
            .filter_map(|f| f.name.to_owned().map(|name| (name, f)))
            .filter_map(|(name, f)| {
                for prefix in self.0.iter() {
                    if let Some(name) = name.strip_prefix(prefix) {
                        return Some((name.to_owned(), f.id()));
                    }
                }
                None
            })
            .collect();

        let rids: HashMap<FunctionId, FunctionId> = imps
            .iter()
            .filter_map(|(name, fid)| {
                fs.get(name).map(|rid| {
                    (
                        fid.to_owned(), // src
                        rid.to_owned(), // dst
                    )
                })
            })
            .collect();

        m.elements.iter_mut().for_each(|el| {
            if let ElementItems::Functions(fids) = el.items.borrow_mut() {
                for fid in fids {
                    if let Some(rid) = rids.get(fid) {
                        *fid = *rid;
                    }
                }
            }
        });

        m.funcs.iter_mut().for_each(|f| {
            if let FunctionKind::Local(f) = &mut f.kind {
                process_block(f, f.entry_block(), &rids);
            }
        });

        Ok(())
    }
}

fn process_block(f: &mut LocalFunction, id: InstrSeqId, rids: &HashMap<FunctionId, FunctionId>) {
    let mut ids = vec![];

    for (instr, _) in f.block_mut(id).instrs.iter_mut() {
        match instr {
            // Replace
            Instr::RefFunc(i) => {
                if let Some(rid) = rids.get(&i.func) {
                    i.func = *rid
                }
            }
            Instr::Call(i) => {
                if let Some(rid) = rids.get(&i.func) {
                    i.func = *rid
                }
            }
            Instr::ReturnCall(i) => {
                if let Some(rid) = rids.get(&i.func) {
                    i.func = *rid
                }
            }

            // Queue
            Instr::Block(i) => {
                ids.push(i.seq);
            }
            Instr::Loop(i) => {
                ids.push(i.seq);
            }
            Instr::IfElse(i) => {
                ids.push(i.consequent);
                ids.push(i.alternative);
            }

            _ => {}
        }
    }

    for id in ids {
        process_block(f, id, rids);
    }
}

struct StartEntry;

impl Strip for StartEntry {
    fn strip(&self, m: &mut Module) -> Result<(), Error> {
        // Skip if already present
        if m.start.is_some() {
            return Ok(());
        }

        // Find and set init point
        if let Some(f) = m.funcs.by_name(PREFIX_INIT) {
            m.start = Some(f);
        }

        Ok(())
    }
}

struct StartExport;

impl Strip for StartExport {
    fn strip(&self, m: &mut Module) -> Result<(), Error> {
        // Search for export
        let eid = m
            .exports
            .iter()
            .filter(|e| e.name.starts_with(PREFIX_INIT))
            .find_map(|e| match e.item {
                ExportItem::Function(_) => Some(e.id()),
                _ => None,
            });

        // Remove export
        if let Some(eid) = eid {
            m.exports.delete(eid);
        }

        Ok(())
    }
}

struct Unused;

impl Strip for Unused {
    fn strip(&self, m: &mut Module) -> Result<(), Error> {
        // Perform a GC pass
        // This will clean up unused imports
        gc::run(m);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Error};
    use walrus::Module;

    use crate::{CallReplace, StartEntry, StartExport, Strip, Unused};

    #[test]
    fn test_add_start_entry() -> Result<(), Error> {
        let wat = r#"
            (module
                (func $_initialize nop)
            )
        "#;

        let bs = wat::parse_str(wat)?;
        let mut m = Module::from_buffer(&bs)?;

        assert!(m.start.is_none());
        StartEntry.strip(&mut m)?;
        assert!(m.start.is_some());

        Ok(())
    }

    #[test]
    fn test_remove_start_export() -> Result<(), Error> {
        let wat = r#"
            (module
                (func $_initialize nop)
                (export "_initialize" (func $_initialize))
            )
        "#;

        let bs = wat::parse_str(wat)?;
        let mut m = Module::from_buffer(&bs)?;

        StartExport.strip(&mut m)?;

        for e in m.exports.iter() {
            if e.name.starts_with("_initialize") {
                return Err(anyhow!("module still contains _initialize export"));
            }
        }

        Ok(())
    }

    #[test]
    fn test_remove_unsued() -> Result<(), Error> {
        let wat = r#"
            (module
                (import "wasi_snapshot_preview1" "proc_exit"
                    (func $__imported_wasi_snapshot_preview1_proc_exit (param i32)))

                (func $_initialize nop)
            )
        "#;

        let bs = wat::parse_str(wat)?;
        let mut m = Module::from_buffer(&bs)?;

        Unused.strip(&mut m)?;

        let i = m.imports.find("wasi_snapshot_preview1", "proc_exit");
        assert!(i.is_none());

        Ok(())
    }

    #[test]
    fn test_replacement() -> Result<(), Error> {
        let wat = r#"
            (module
                (import "wasi_snapshot_preview1" "proc_exit"
                    (func $__imported_wasi_snapshot_preview1_proc_exit (param i32)))

                (func $__prefix_proc_exit (param i32) nop)

                (func $_initialize
                    i32.const 0
                    call $__imported_wasi_snapshot_preview1_proc_exit
                )
            )
        "#;

        let bs = wat::parse_str(wat)?;
        let mut m = Module::from_buffer(&bs)?;

        CallReplace(vec!["__prefix_".to_string()]).strip(&mut m)?;

        Ok(())
    }
}
