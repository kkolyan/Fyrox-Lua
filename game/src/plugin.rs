use fyrox::plugin::{Plugin, PluginContext, PluginRegistrationContext};
use mlua::{Function, Lua};
use fyrox::event::Event;
use fyrox::gui::message::UiMessage;
use std::path::Path;
use std::sync::Arc;
use fyrox::core::pool::Handle;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::Visit;
use fyrox::scene::Scene;
use fyrox::{
    core::reflect::prelude::*,
    core::visitor::prelude::*,
};
use fyrox::core::log::Log;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::{Script};
use fyrox::walkdir::{DirEntry, WalkDir};
use crate::lua_utils::log_error;
use crate::script::ScriptMetadata;
use crate::script::LuaScript;
use crate::script::ScriptDefinition;

#[derive(Visit, Reflect, Debug)]
pub struct LuaPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub lua: &'static Lua,
}

impl Default for LuaPlugin {
    fn default() -> Self {
        LuaPlugin {
            lua: Box::leak(Box::new(Lua::new())),
        }
    }
}

impl Plugin for LuaPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        // mlua has approach with lifetimes that makes very difficult storing Lua types
        // here and there in Rust. But we need a single Lua VM instance for the whole life
        // of game process, so that's ok to make it 'static.
        let lua: &'static mut Lua = Box::leak(Box::new(Lua::new()));

        log_error(
            "set 'package.path'",
            lua.load("package.path = 'scripts/?.lua;scripts/?/init.lua'"
            ).eval::<()>());

        for entry in WalkDir::new("scripts").into_iter().flatten() {
            load_script(&context, lua, &entry);
        }
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        Lua::new().load("print('hello Fyrox'); print(_VERSION)").eval::<()>().unwrap();
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }
}

fn load_script(context: &PluginRegistrationContext, lua: &mut Lua, entry: &DirEntry) {
    if !entry.file_type().is_file() {
        return;
    }
    let metadata = ScriptMetadata::parse_file(entry.path());
    let metadata = match metadata {
        Ok(it) => it,
        Err(errs) => {
            for err in errs {
                Log::err(format!("failed to load script from file {}: {}", &entry.path().to_string_lossy(), err));
            }
            return;
        }
    };

    let class_loading = lua.load("return function(x) require(x) end")
        .eval::<Function>()
        .and_then(|it| it.call::<_, ()>(metadata.class.clone()));
    match class_loading {
        Ok(_) => {}
        Err(err) => {
            Log::err(format!("Failed to load Lua class {:?}: {}", &metadata.class, err));
            return;
        }
    }

    let name = metadata.class;
    let uuid = metadata.uuid;

    let assembly_name = "scripts/**.lua";
    let definition = Arc::new(ScriptDefinition {
        metadata,
        assembly_name,
    });

    context
        .serialization_context
        .script_constructors
        .add_custom(
            uuid,
            ScriptConstructor {
                constructor: Box::new(move || {
                    Script::new(LuaScript::new(&definition))
                }),
                name: name.to_string(),
                source_path: entry.path().to_string_lossy().to_string().leak(),
                assembly_name,
            },
        );
    Log::info(format!("script registered: {}", entry.path().to_string_lossy()));
}
