//! Executor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox::engine::executor::Executor;

fn main() {
    Log::set_verbosity(MessageKind::Warning);
    let mut executor = Executor::new();

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        executor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use fyrox_lua::plugin::LuaPlugin;
        executor.add_plugin(LuaPlugin::default());
    }

    executor.run()
}
