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
    db::{DefDatabase, HirDatabase}, import_map::Query, Adt, ChangeWithProcMacros, Crate, HasVisibility, HirDisplay, HirFileIdExt, Impl, ImportPathConfig, ItemInNs, Module, ModuleDef, Semantics, SemanticsImpl, Type, Visibility
};
use hir_ty::{
    method_resolution::{self, LookupMode, VisibleFromModule},
    TraitEnvironment,
};
use ide::{AnalysisHost, Edition, RootDatabase, SourceRoot};
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use paths::AbsPathBuf;
use project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use syntax::{ast::{self, HasModuleItem, Item}, SyntaxKind, SyntaxTreeBuilder};
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

    let crates = Crate::all(db);

    for crate_ in crates {
        let root_file = crate_.root_file(db);
        let path = vfs.file_path(root_file);
        let path = path.as_path().unwrap().to_string();
        //C:\Users\kkolyan\.cargo\registry\src\index.crates.io-6f17d22bba15001f\nalgebra-0.32.6\src\lib.rs
        if path.contains("nalgebra-0.32.6") {
            println!("{}", path);
            println!();

            print_using_hir_impls(db, crate_, &vfs);
        }
    }
}

fn print_custom_2(db: &RootDatabase, crate_: Crate, vfs: &Vfs) {
    print_custom_module_2(db, crate_.root_module(), vfs);
}

fn print_custom_module_2(db: &RootDatabase, mod_: Module, vfs: &Vfs) {
    for item in mod_.children(db) {
        print_custom_module_2(db, item, vfs);
    }

    let file = mod_.as_source_file_id(db);
    if let Some(file_id) = file {
    
        let file = db.parse(file_id);
        match file.ok() {
            Ok(file) => {
                for item in file.items() {
                    if let Item::Impl(impl_) = item {
                        if let Some(trait_) = impl_.trait_() {
                            print!("  {}", trait_);
                            if let Some(ty) = impl_.self_ty() {
                                print!(" for {}", ty);
                            }
                            println!()
                        }
                    }
                }
            },
            Err(err) => {
                println!("failed to parse: {}: {:?}", vfs.file_path(file_id), err);
            },
        };
    }
}

fn print_custom(db: &RootDatabase, crate_: Crate) {
    print_custom_module(db, crate_.root_module());
}

fn print_custom_module(db: &RootDatabase, mod_: Module) {
    for item in mod_.children(db) {
        print_custom_module(db, item);
    }

    for impl_ in mod_.impl_defs(db) {
        if let Some(trait_) = impl_.trait_(db) {
            println!("  {}", trait_.display(db));
            println!("  {}", impl_.self_ty(db).display(db));
            println!();
        }
    }
}

fn print_using_hir_impls(db: &RootDatabase, crate_: Crate, _: &Vfs) {
    let impls = Impl::all_in_crate(db, crate_);
    for impl_ in impls {
        if let Some(trait_) = impl_.trait_(db) {
            println!("  {}", trait_.display(db));
            println!("  {}", impl_.self_ty(db).display(db));
            println!();
        }
    }
}