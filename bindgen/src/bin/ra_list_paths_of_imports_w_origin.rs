use std::{
    collections::BTreeSet,
    fs,
    ops::Deref,
    path::{self, PathBuf},
    str::FromStr,
    time::{Duration, SystemTime},
};

use bindgen::stopwatch::start_watch;
use itertools::Itertools;
use base_db::Upcast;
use base_db::{FileSet, SourceDatabase};
use hir::{
    db::{DefDatabase, HirDatabase},
    import_map::Query,
    ChangeWithProcMacros, Crate, HasVisibility, HirFileIdExt, ImportPathConfig, ModuleDef, Type,
    Visibility,
};
use ide::{AnalysisHost, SourceRoot};
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use paths::AbsPathBuf;
use project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use to_vec::ToVec;

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

    // let mut host = AnalysisHost::with_database(db);

    let db: &(dyn HirDatabase + 'static) = root_db.upcast();
    let crates = &Crate::all(db);

    let fyrox_lua = crates
        .iter()
        .find(|it| it.display_name(db).unwrap().to_string() == "fyrox_lua")
        .unwrap();

    let _asd = start_watch("imports");
    let mut imports = BTreeSet::new();
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
                        match m {
                            ModuleDef::Module(_) => continue,
                            ModuleDef::Function(f) => "Function",
                            ModuleDef::Adt(_) => "Adt",
                            ModuleDef::Const(_) => "Const",
                            ModuleDef::Static(_) => "Static",
                            ModuleDef::Trait(_) => "Trait",
                            ModuleDef::TypeAlias(_) => "TypeAlias",
                            unsupported => todo!("unsupported def: {:?}", unsupported),
                        };


                        let path = fyrox_lua.root_module().find_use_path(
                            &root_db,
                            m,
                            hir::PrefixKind::Plain,
                            ImportPathConfig {
                                prefer_no_std: true,
                                prefer_prelude: true,
                            },
                        );
                        match path {
                            Some(path) => {

                                let visible_path = path.display(&root_db);
                                
                                let crate_ = m.module(db).unwrap().krate();
;
                                let real_path = format!(
                                    "{}::{}",
                                    crate_.display_name(db).unwrap(),
                                    m.canonical_path(db).unwrap()
                                );
                                imports.insert(format!("{}: {}", visible_path, real_path));
                            }
                            None => {
                                // for some reason, here are methods of traits and structs.
                                // I don't see more explicit or easier way to filter out them
                            }
                        }
                    }
                    itertools::Either::Right(_m) => {}
                }
            }
        }
    }

    let imports = imports
        .into_iter()
        // .map(|it| format!("use {} as {};", it, it.replace("::", "_")))
        .to_vec();

    fs::write(
        "bindgen/fyrox_all_static_symbols_w_origin.txt",
        imports.join("\n"),
    )
    .unwrap();

}

fn try_identify_module_def(m: &ModuleDef, db: &dyn HirDatabase) -> String {
    let def_type = match m {
        ModuleDef::Module(_) => "Module",
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
            format!("{}/{}", def_type, it)
        }
        None => match m {
            ModuleDef::Module(it) => {
                if it.is_crate_root() {
                    "crate_root".to_string()
                } else {
                    panic!("WTF")
                }
            }
            _ => panic!("WTF"),
        },
    }
}
