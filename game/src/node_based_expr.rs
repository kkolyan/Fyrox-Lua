use crate::lua_reflect_bindings::LuaTableKey;
use crate::lua_reflect_bindings::ReflectUserData;
use crate::lua_utils::OptionX;
use fyrox::core::pool::Handle;
use fyrox::core::reflect::Reflect;
use fyrox::scene::node::Node;
use fyrox::script::ScriptContext;
use mlua::Result as LuaResult;

/// Lua UserData to represent a value transitively owned by Fyrox Node Pool.
pub struct NodeBasedExpression {
    handle: Handle<Node>,
    path: Vec<LuaTableKey>,
}

impl ReflectUserData for NodeBasedExpression {
    type RefType = NodeBasedExpression;

    fn address<'a>(
        &'a mut self,
        sc: &'a mut ScriptContext,
    ) -> LuaResult<(&'a mut dyn Reflect, &[LuaTableKey])> {
        Ok((
            sc.scene
                .graph
                .try_get_mut(self.handle)
                .map(|it| {
                    let it: &mut dyn Reflect = it;
                    it
                })
                .lua_ok_or("Node is not present on scene anymore")?,
            &self.path,
        ))
    }

    fn create_sibling(&self, absolute_path: Vec<LuaTableKey>) -> LuaResult<Self::RefType> {
        Ok(NodeBasedExpression {
            handle: self.handle,
            path: absolute_path,
        })
    }
}
