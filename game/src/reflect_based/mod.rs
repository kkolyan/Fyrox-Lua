use mlua::{FromLua, Lua, UserDataFields, UserDataMethods, Value};
use mlua::prelude::LuaResult;

use crate::lua_error;

pub mod reflect_lua_bindings;
mod reflect;
mod conversion;

pub fn demo() {
}


#[derive(Clone)]
pub enum LuaTableKey {
    Number(usize),
    // TODO isn't it too heavy?
    String(String),
}

pub(crate) fn format_path(path: &[LuaTableKey]) -> String {
    let mut s = String::new();
    for it in path {
        s.push_str(match it {
            LuaTableKey::Number(i) => format!("[{}]", i),
            LuaTableKey::String(k) => format!(".{}", k),
        }.as_str());
    }
    s
}

impl FromLua<'_> for LuaTableKey {
    fn from_lua(value: Value, _lua: & Lua) -> LuaResult<Self> {
        match value {
            Value::Integer(i) => Ok(LuaTableKey::Number(i as usize)),
            Value::Number(i) => Ok(LuaTableKey::Number(i as usize)),
            Value::String(key) => Ok(LuaTableKey::String(key.to_str()?.to_string())),
            invalid => Err(lua_error!("Invalid LuaTableKey: {:?}", invalid)),
        }
    }
}

