use std::{path::PathBuf, str::FromStr};

use bindgen::stopwatch::start_watch;
use base_db::Upcast;
use hir::{db::HirDatabase, Crate, Module, ModuleDef};
use ide::RootDatabase;
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
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

	let _examine_type_watch = start_watch("examine type");

    let db: &(dyn HirDatabase + 'static) = root_db.upcast();

    examine_symbol("fyrox_impl::scene::node::Node", db);
}

fn examine_symbol(real_path: &str, db: &dyn HirDatabase) {
    let path_parts = real_path.split("::").to_vec();
    let crate_name = path_parts[0];
    let symbol_name = path_parts[path_parts.len() - 1];
    let mod_path = &path_parts[1..path_parts.len() - 1];
    let crate_ = Crate::all(db)
        .into_iter()
        .find(|it| it.display_name(db).unwrap().to_string() == crate_name)
        .unwrap();

	let resolve_mod_watch = start_watch("resolve mod");

    let mut mod_ = crate_.root_module();
    for path_element in mod_path {
        mod_ = mod_
            .children(db)
            .find(|it| it.name(db).unwrap().as_str() == Some(path_element))
            .unwrap();
    }
	resolve_mod_watch.force_complete();

	let _resolve_decl_watch = start_watch("resolve decl");

    let decl = mod_.declarations(db)
        .into_iter()
        .find(|it| it.name(db).unwrap().as_str() == Some(symbol_name));
	match decl {
		Some(_) => {},
		None => {
			println!("failed to resolve symbol {} in {}::{}", symbol_name, crate_name, mod_path.to_vec().join("::"));
		},
	}
}
