use std::{
    collections::BTreeSet,
    fs,
    ops::Deref,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime},
};

use bindgen::stopwatch::start_watch;
use itertools::Itertools;
use base_db::{FileSet, SourceDatabase};
use hir::{
    db::{DefDatabase, HirDatabase},
    import_map::Query,
    ChangeWithProcMacros, Crate, HasVisibility, HirFileIdExt, ModuleDef, Type, Visibility,
};
use ide::{AnalysisHost, SourceRoot};
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use paths::AbsPathBuf;
use project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use to_vec::ToVec;
use base_db::Upcast;

fn main() {
    let ws_loading_watch = start_watch("workspace loading");
    let (root_db, vfs, _proc_macro_server) = load_workspace_at(
        &PathBuf::from_str("c:/dev/rust/fyrox_lua/game/Cargo.toml").unwrap(),
        &Default::default(),
        &LoadCargoConfig {
            load_out_dirs_from_check: true,
            with_proc_macro_server: ProcMacroServerChoice::Sysroot,
            prefill_caches: false,
        },
        &|it| println!("workspace loading progress: {}", it),
    )
    .unwrap();
    ws_loading_watch.force_complete();

    for krate in root_db.crate_graph().iter() {
        let data = &root_db.crate_graph()[krate];
        let name = data.display_name.as_ref().unwrap().canonical_name();
        // let path = vfs.file_path(data.root_file_id).to_string();
        if name.contains("fyrox") {
            println!("{}", name)
        }
    }

    // let mut host = AnalysisHost::with_database(db);

    let db: &(dyn HirDatabase + 'static) = root_db.upcast();
    let crates = Crate::all(db);

    let _asd = start_watch("imports");
    let mut imports = Vec::new();
    for crate_ in crates {
        let crate_name = crate_.display_name(db);
        let crate_name = crate_name.as_ref().unwrap().canonical_name();
        if crate_name.eq("fyrox") {
            println!("{}", crate_name);
            let results =
                crate_.query_external_importables(&root_db, Query::new("".to_string()).prefix());

            for it in results {
                match it {
                    itertools::Either::Left(m) => {
                        if m.visibility(db) != Visibility::Public {
                            continue;
                        }
                        let def_type = match m {
                            ModuleDef::Module(_) => continue,
                            ModuleDef::Function(_) => "Function",
                            ModuleDef::Adt(_) => "Adt",
                            ModuleDef::Const(_) => "Const",
                            ModuleDef::Static(_) => "Static",
                            ModuleDef::Trait(_) => "Trait",
                            ModuleDef::TypeAlias(_) => "TypeAlias",
                            unsupported => todo!("unsupported def: {:?}", unsupported),
                        };
                        match m.canonical_path(db) {
                            Some(it) => {
                                imports.push(format!("{} {}::{}", def_type, crate_name, it))
                            }
                            None => match m {
                                ModuleDef::Module(it) => {
                                    let src = it.definition_source_range(db);
                                    imports.push(format!(
                                        "UNRESOLVED: {}:: crate_root: {} {:?} ({:?})",
                                        crate_name,
                                        it.is_crate_root(),
                                        src,
                                        vfs.file_path(src.file_id.original_file(&root_db))
                                    ))
                                }
                                _ => panic!("WTF"),
                            },
                        }
                    }
                    itertools::Either::Right(m) => {}
                }
            }
        }
    }
    imports.sort();
    fs::write("bindgen/ra.txt", imports.join("\n")).unwrap();

    // it contains all fyrox modules
    // fs::write("graph.txt", host.analysis().view_crate_graph(true).unwrap().unwrap()).unwrap();

    // host.analysis().fetch_crates()

    // for krate in state.fetch_crates().unwrap().iter() {
    // 	if krate.name.as_ref().unwrap().contains("fyrox") {
    // 		println!("{}", krate.name.as_ref().unwrap());
    // 	}
    // }
    // for item in host.raw_database().crate_graph().iter() {
    // 	item.
    // }

    // println!("{:?}", host);
}
