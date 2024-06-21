use mlua::prelude::LuaResult;
use std::cell::RefCell;

use crate::lua_utils::OptionX;
use fyrox::script::ScriptContext;

type StaticSc = ScriptContext<'static, 'static, 'static>;

thread_local! {
    static SCRIPT_CONTEXT: RefCell<Option<&'static mut StaticSc>> = RefCell::new(None);
}
const SC_404: &str = "Fyrox ScriptContext is not available outside of main thread and ";

/// the way to access Fyrox engine API from the Rust functions called from Lua
pub(crate) fn with_script_context<T, F>(f: F) -> LuaResult<T>
where
    F: FnOnce(&mut StaticSc) -> LuaResult<T>,
{
    SCRIPT_CONTEXT.with_borrow_mut(|sc| {
        let sc = sc.as_mut().lua_ok_or(SC_404)?;
        f(sc)
    })
}
