use mlua::{Number, Result as LuaResult, Value};

use fyrox::core::reflect::Reflect;

use crate::reflect_based::{LuaTableKey};

pub fn get_terminal_node_of_path_mut(
    source: &mut dyn Reflect,
    path: &[LuaTableKey],
    result: &mut dyn FnMut(Result<&mut dyn Reflect, &str>)
) {
    if path.is_empty() {
        result(Ok(source));
    } else if path.len() > 1 {
        get_value_by_lua_key_mut(
            source,
            path.first().unwrap(),
            &mut |value| {
                match value {
                    Ok(value) => get_terminal_node_of_path_mut(value, &path[1..], result),
                    Err(error) => result(Err(error)),
                }
            },
        );
    } else {
        get_value_by_lua_key_mut(
            source,
            path.first().unwrap(),
            result,
        );
    }
}

fn get_value_by_lua_key_mut(
    source: &mut dyn Reflect,
    key: &LuaTableKey,
    result: &mut dyn FnMut(Result<&mut dyn Reflect, &str>)
) {
    match key {
        LuaTableKey::Number(i) => {
            source.as_array_mut(&mut |array| {
                match array {
                    None => {
                        result(Err("index used to access non-indexable object"));
                    }
                    Some(array) => {
                        if let Some(it) = array.reflect_index_mut(*i) {
                            result(Ok(it));
                        }
                    }
                }
            });
        }
        LuaTableKey::String(key) => {
            source.field_mut(&key, &mut |value| {
                if let Some(it) = value {
                    result(Ok(it));
                };
            });
        }
    }
}
