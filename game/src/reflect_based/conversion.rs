use fyrox::core::ImmutableString;
use fyrox::core::reflect::Reflect;
use mlua::{Lua, Result as LuaResult, Value};
use std::path::PathBuf;
use fyrox::core::algebra::Vector3;
use crate::lua_bindings::Vector3Ud;
use crate::lua_error;
use crate::lua_utils::OptionX;

pub trait FyroxAndLua {
    /// None if type not supported
    /// Some(Err) if type is supported, but something went wrong
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>>;

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>>;
}


macro_rules! fyrox_and_lua_numbers {
    ($($type:ty),*) => {
        $(
            impl FyroxAndLua for $type {
                fn fyrox_to_lua<'a>(p: &mut dyn Reflect, _lua: &'a Lua) -> Option<mlua::Result<mlua::Value<'a>>> {
                    let mut result: Option<mlua::Result<mlua::Value<'a>>> = None;
                    p.downcast_ref::<Self>(&mut |p| {
                        result = p.map(|it| Ok(mlua::Value::Number(*it as mlua::Number)))
                    });
                    result
                }

                fn lua_to_fyrox(p: &mut dyn Reflect, value: &mlua::Value) -> Option<mlua::Result<()>> {
                    let mut result: Option<mlua::Result<()>> = None;
                    p.downcast_mut::<Self>(&mut |p| {
                        result = p.map(|it| {
                            let s = crate::lua_utils::ValueX::as_f64_tolerant(value).lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?;
                            *it = s as Self;
                            Ok(())
                        })
                    });
                    result
                }
            }
        )*
    };
}
fyrox_and_lua_numbers!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);


impl FyroxAndLua for bool {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, _lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<bool>(&mut |p| {
            result = p.map(|it| Ok(Value::Boolean(*it)))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                *it = value.as_boolean().lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?;
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for String {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| lua.create_string(it).map(Value::String))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                *it = value.as_str().lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?.to_string();
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for PathBuf {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| {
                it.to_str()
                    .map(|s| lua.create_string(s).map(Value::String))
                    .unwrap_or_else(|| Err(lua_error!("")))
            })
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                let s = value.as_str().lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?.to_string();
                *it = Self::from(s);
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for ImmutableString {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| lua.create_string(it.as_str()).map(Value::String))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                let s = value.as_str().lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?.to_string();
                *it = Self::from(s);
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for Vector3<f32> {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| lua.create_userdata(Vector3Ud(*it)).map(Value::UserData))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                let s = value.as_userdata()
                    .lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?
                    .borrow::<Vector3Ud>()?;
                *it = s.0;
                Ok(())
            })
        });
        result
    }
}
