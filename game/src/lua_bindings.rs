use std::ops::AddAssign;
use fyrox::core::algebra::Vector3;
use mlua::{UserData, UserDataFields, UserDataMethods, UserDataRef};
use crate::lua_reflect_bindings;
use crate::node_based_expr::NodeBasedExpression;
use crate::script::{LuaScript, LuaScriptBasedExpr};

pub struct Vector3Ud(pub Vector3<f32>);

impl UserData for Vector3Ud {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_lua, this| Ok(this.0.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.0.y));
        fields.add_field_method_get("z", |_lua, this| Ok(this.0.z));
        fields.add_field_method_set("x", |_lua, this, v| {this.0.x = v;Ok(())});
        fields.add_field_method_set("y", |_lua, this, v| {this.0.y = v;Ok(())});
        fields.add_field_method_set("z", |_lua, this, v| {this.0.z = v;Ok(())});
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("normalize_mut", |_lua, this, _args: ()| {
            this.0.normalize_mut();
            Ok(())
        });
        methods.add_method("normalize", |_lua, this, _args: ()| {
            Ok(Vector3Ud(this.0.normalize()))
        });
        methods.add_method("magnitude", |_lua, this, _args: ()| {
            Ok(this.0.magnitude())
        });
        methods.add_method_mut("add_assign", |_lua, this, other: UserDataRef<Vector3Ud>| {
            this.0.add_assign(other.0);
            Ok(())
        });
    }
}

impl UserData for LuaScript {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_reflect_bindings::populate_reflect_lua_bindings(methods);
    }
}

impl UserData for LuaScriptBasedExpr {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_reflect_bindings::populate_reflect_lua_bindings(methods);
    }
}

impl UserData for NodeBasedExpression {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_reflect_bindings::populate_reflect_lua_bindings(methods);
    }
}

pub struct EngineUd {

}

impl UserData for EngineUd {

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("cast_ray", |_lua, _this, _args: ()| Ok(()));
        methods.add_method("send_message", |_lua, _this, _args: ()| Ok(()));
    }
}
