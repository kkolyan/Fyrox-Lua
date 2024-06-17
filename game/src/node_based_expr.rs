use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use mlua::{Result as LuaResult, UserData, UserDataMethods};
use fyrox::core::reflect::Reflect;
use fyrox::script::ScriptContext;
use crate::lua_reflect_bindings;
use crate::lua_reflect_bindings::{LuaTableKey, ReflectUserData};
use crate::lua_utils::OptionX;

/// Lua UserData to represent a value transitively owned by Fyrox Node Pool.
pub struct NodeBasedExpression {
    handle: Handle<Node>,
    path: Vec<LuaTableKey>,
}

impl ReflectUserData for NodeBasedExpression {
    type RefType = NodeBasedExpression;

    fn address<'a>(&'a mut self, sc: &'a mut ScriptContext) -> LuaResult<(&'a mut dyn Reflect, &[LuaTableKey])> {
        Ok((
            sc.scene.graph.try_get_mut(self.handle)
                .map(|it| {
                    let it: &mut dyn Reflect = it;
                    it
                })
                .lua_ok_or("Node is not present on scene anymore")?,
            &self.path
        ))
    }

    fn create_sibling(&self, absolute_path: Vec<LuaTableKey>) -> LuaResult<Self::RefType> {
        Ok(NodeBasedExpression { handle: self.handle, path: absolute_path })
    }
}
