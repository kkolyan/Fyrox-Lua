use std::{
    any::Any,
    collections::{BTreeSet, HashSet},
    fs,
    ops::Deref,
    path::{self, PathBuf},
    str::FromStr,
    time::{Duration, SystemTime},
};

use bindgen::stopwatch::start_watch;
use itertools::Itertools;
use base_db::{FileSet, SourceDatabase};
use base_db::{SourceDatabaseExt, Upcast};
use hir::{
    db::{DefDatabase, HirDatabase}, import_map::Query, known::Ok, Adt, ChangeWithProcMacros, Crate, HasVisibility, HirFileIdExt, Impl, ImportPathConfig, ItemInNs, ModuleDef, Semantics, SemanticsImpl, Type, Visibility
};
use hir_ty::{
    method_resolution::{self, LookupMode, VisibleFromModule},
    TraitEnvironment,
};
use ide::{AnalysisHost, Edition, RootDatabase, SourceRoot};
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use paths::AbsPathBuf;
use project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use syntax::{ast, SyntaxKind, SyntaxTreeBuilder};
use vfs::Vfs;
use to_vec::ToVec;

fn main() {
    let ws_loading_watch = start_watch("workspace loading");
    let (ref db, vfs, _proc_macro_server) = load_workspace_at(
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

    let crates = &Crate::all(db);

    let fyrox_lua = crates
        .iter()
        .find(|it| it.display_name(db).unwrap().to_string() == "fyrox_lua")
        .unwrap();

    let _asd = start_watch("imports");
    // let mut imports_set = HashSet::new();
    let mut imports = vec![];
    for crate_ in crates {
        let crate_name = crate_.display_name(db);
        let crate_name = crate_name.as_ref().unwrap().canonical_name();
        if crate_name.eq("fyrox") {
            println!("{}", crate_name);
            let results =
                crate_.query_external_importables(db, Query::new("".to_string()).prefix());

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

                        let path = visible_path(fyrox_lua, db, m);
                        match path {
                            Some(path) => {
                                let visible_path = path;

                                let crate_ = m.module(db).unwrap().krate();
                                let real_path = format!(
                                    "{}::{}",
                                    crate_.display_name(db).unwrap(),
                                    m.canonical_path(db).unwrap()
                                );
                                let title = format!("{}: {}", visible_path, real_path);
                                if title != "fyrox::core::algebra::Vector3: nalgebra::base::alias::Vector3" {
                                    continue;
                                }
                                // if imports_set.insert(title.clone()) {
                                    imports.push(title);

                                    examine(&m, db, &mut imports, fyrox_lua);
                                // }
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

    fs::write("bindgen/examine_all_types.txt", imports.join("\n")).unwrap();
}

fn examine(m: &ModuleDef, db: &RootDatabase, results: &mut Vec<String>, user_crate: &Crate) {
    let s = Semantics::new(db);
    match m {
        ModuleDef::Function(f) => {}
        ModuleDef::Adt(adt) => match adt {
            Adt::Struct(it) => {
                examine_type(db, it.ty(db), results, user_crate);
            }
            Adt::Union(it) => {
                examine_type(db, it.ty(db), results, user_crate);
            }
            Adt::Enum(it) => {
                examine_type(db, it.ty(db), results, user_crate);
            }
        },
        ModuleDef::TypeAlias(alias) => {
            examine_type(db, alias.ty(db), results, user_crate);
        }
        _ => {}
    }
}

fn examine_type(db: &RootDatabase, it: Type, results: &mut Vec<String>, user_crate: &Crate) {
    let mut local = vec![];
    let impls = Impl::all_for_type(db, it);
    for impl_ in impls {
        for ele in impl_.items(db) {
            let trait_ = impl_
                .trait_(db)
                .map(|it| {
                    visible_path(user_crate, db, ModuleDef::Trait(it))
                    .unwrap_or_else(|| format!("UNRESOLVED: {}", it.name(db).as_str().unwrap()))
                })
                .map(|it| format!(" ({})", it))
                .unwrap_or("".to_string());
            local.push(format!(
                "    {}{}",
                ele.name(db).unwrap().as_str().unwrap(),
                trait_
            ));
        }
    }
    local.sort();
    results.extend_from_slice(&local);
}

fn visible_path(user_crate: &Crate, db: &RootDatabase, m: impl Into<ItemInNs>) -> Option<String> {
    
    let path = user_crate.root_module().find_use_path(
        db,
        m,
        hir::PrefixKind::Plain,
        ImportPathConfig {
            prefer_no_std: true,
            prefer_prelude: true,
        },
    );
    Some(format!("{}", path?.display(db)))
}

fn try_to_get_all() {

    // let canonical = hir_ty::replace_errors_with_variables(&it.ty(db).into());
    // method_resolution::iterate_method_candidates_dyn(
    //     &canonical,
    //     db,
    //     TraitEnvironment::empty(m.module(db).unwrap().krate().),
    //      &Default::default(),
    //      VisibleFromModule::None,
    //      None,
    //      LookupMode::MethodCall,
    //      || {

    //      }
    // );

    // it.ty(db).iterate_method_candidates_with_traits(
    //     db,
    //     &scope,
    //     &Default::default(),
    //     None,
    //     None,
    //     |it| Some(it),
    // );
}
