//! Wrapper for hot-reloadable plugin.
use fyrox_lua::fyrox::plugin::Plugin;
use fyrox_lua::plugin::LuaPlugin;

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(LuaPlugin::default())
}
