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

    for crate_ in crates.iter() {
        let root_file = crate_.root_file(db);
        let path = vfs.file_path(root_file);
        println!("{}", path.as_path().unwrap());
    }
}