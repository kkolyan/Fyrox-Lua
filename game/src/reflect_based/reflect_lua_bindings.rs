use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use mlua::{MetaMethod, UserData, UserDataFields, UserDataMethods, Value};
use std::path::PathBuf;
use fyrox::core::ImmutableString;
use crate::reflect_based::{format_path, LuaTableKey};
use crate::{lua_error, SC_404, SCRIPT_CONTEXT};
use crate::lua_utils::OptionX;
use crate::reflect_based::conversion::FyroxAndLua;
use crate::reflect_based::reflect::get_terminal_node_of_path_mut;

/// Reference to a value owned by node.
pub struct NodeRelatedData {
    handle: Handle<Node>,
    path: Vec<LuaTableKey>,
}

impl UserData for NodeRelatedData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, key: LuaTableKey| {
            SCRIPT_CONTEXT.with(|ctx| {
                let sc = &mut ctx.borrow_mut();
                let sc = sc.as_mut()
                    .lua_ok_or(SC_404)?;
                let node = sc.scene.graph.try_get_mut(this.handle)
                    .lua_ok_or("Node is not present on scene anymore")?;

                let mut extended_path = this.path.clone();
                extended_path.push(key);

                let mut lua_result = None;

                get_terminal_node_of_path_mut(node, &extended_path, &mut |result| {
                    match result {
                        Ok(result) => {
                            lua_result = None
                                .or_else(|| String::fyrox_to_lua(result, lua))
                                .or_else(|| PathBuf::fyrox_to_lua(result, lua))
                                .or_else(|| ImmutableString::fyrox_to_lua(result, lua))
                                .or_else(|| i8::fyrox_to_lua(result, lua))
                                .or_else(|| i16::fyrox_to_lua(result, lua))
                                .or_else(|| i32::fyrox_to_lua(result, lua))
                                .or_else(|| i64::fyrox_to_lua(result, lua))
                                .or_else(|| u8::fyrox_to_lua(result, lua))
                                .or_else(|| u16::fyrox_to_lua(result, lua))
                                .or_else(|| u32::fyrox_to_lua(result, lua))
                                .or_else(|| u64::fyrox_to_lua(result, lua))
                                .or_else(|| usize::fyrox_to_lua(result, lua))
                                .or_else(|| isize::fyrox_to_lua(result, lua))
                                .or_else(|| f32::fyrox_to_lua(result, lua))
                                .or_else(|| f64::fyrox_to_lua(result, lua))
                                .or_else(|| bool::fyrox_to_lua(result, lua))
                            ;
                        }
                        Err(err) => lua_result = Some(Err(lua_error!("failed to evaluate path {}: {}", format_path(&extended_path), err)))
                    }
                });

                lua_result.unwrap_or_else(|| {
                    let lazy_expression = NodeRelatedData { handle: this.handle, path: extended_path };
                    lua.create_userdata(lazy_expression).map(Value::UserData)
                })
            })
        });
        methods.add_meta_method(MetaMethod::NewIndex, |lua, this, (key, value): (LuaTableKey, Value)| {
            SCRIPT_CONTEXT.with(|ctx| {
                let ctx = &mut ctx.borrow_mut();
                let ctx = ctx.as_mut()
                    .lua_ok_or(SC_404)?;
                let node = ctx.scene.graph.try_get_mut(this.handle)
                    .lua_ok_or("Node is not present on scene anymore")?;

                let mut extended_path = this.path.clone();
                extended_path.push(key);

                let value = &value;

                let mut lua_result = None;


                get_terminal_node_of_path_mut(node, &extended_path, &mut |result| {
                    match result {
                        Ok(result) => {
                            lua_result = None
                                .or_else(|| String::lua_to_fyrox(result, value))
                                .or_else(|| PathBuf::lua_to_fyrox(result, value))
                                .or_else(|| ImmutableString::lua_to_fyrox(result, value))
                                .or_else(|| i8::lua_to_fyrox(result, value))
                                .or_else(|| i16::lua_to_fyrox(result, value))
                                .or_else(|| i32::lua_to_fyrox(result, value))
                                .or_else(|| i64::lua_to_fyrox(result, value))
                                .or_else(|| u8::lua_to_fyrox(result, value))
                                .or_else(|| u16::lua_to_fyrox(result, value))
                                .or_else(|| u32::lua_to_fyrox(result, value))
                                .or_else(|| u64::lua_to_fyrox(result, value))
                                .or_else(|| usize::lua_to_fyrox(result, value))
                                .or_else(|| isize::lua_to_fyrox(result, value))
                                .or_else(|| f32::lua_to_fyrox(result, value))
                                .or_else(|| f64::lua_to_fyrox(result, value))
                                .or_else(|| bool::lua_to_fyrox(result, value))
                            ;
                        }
                        Err(err) => lua_result = Some(Err(lua_error!("failed to evaluate path {}: {}", format_path(&extended_path), err)))
                    }
                });

                lua_result.unwrap_or_else(|| Err(lua_error!("Type is not settable")))
            })
        });
    }
}
