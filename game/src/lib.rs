//! Game project.
pub mod plugin;
pub mod script;
pub mod reflect_based;
pub(crate) mod lua_utils;
pub(crate) mod stacktrace;
pub mod lua_bindings;

use std::cell::RefCell;
use fyrox::{
    core::pool::Handle, core::reflect::prelude::*, core::visitor::prelude::*,
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;
use mlua::Lua;

// Re-export the engine.
pub use fyrox;
use fyrox::core::pool::Ref;
use fyrox::script::ScriptContext;


thread_local! {
    pub(crate) static SCRIPT_CONTEXT: RefCell<Option<&'static mut ScriptContext<'static, 'static, 'static>>> = RefCell::new(None);
}

pub const SC_404: &'static str = "Fyrox ScriptContext is not available outside of main thread and ";
