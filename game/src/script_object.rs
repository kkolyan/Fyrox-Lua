use std::{any::TypeId, sync::Arc};

use fyrox::core::reflect::{FieldInfo, Reflect};

use crate::{lua_error, lua_reflect_bindings::{LuaTableKey, ReflectUserData}, reflect_base, script::{LuaScriptBasedExpr, ScriptFieldValue}, script_def::{ScriptDefinition, ScriptFieldValueType}};



#[derive(Debug, Clone)]
pub struct ScriptObject {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue>,
}

impl ScriptObject {
    pub(crate) fn new(def: &Arc<ScriptDefinition>) -> Self {
        ScriptObject {
            def: def.clone(),
            values: def
                .metadata
                .fields
                .iter()
                .map(|it| match it.ty {
                    ScriptFieldValueType::Number => {
                        ScriptFieldValue::Number(Default::default())
                    }
                    ScriptFieldValueType::String => {
                        ScriptFieldValue::String(Default::default())
                    }
                    ScriptFieldValueType::Bool => ScriptFieldValue::Bool(Default::default()),
                    ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Vector3 => {
                        ScriptFieldValue::Vector3(Default::default())
                    }
                })
                .collect(),
        }
    }
}


impl ReflectUserData for ScriptObject {
    type RefType = LuaScriptBasedExpr;

    fn address<'a>(
        &'a mut self,
        sc: &'a mut fyrox::script::ScriptContext,
    ) -> mlua::Result<(&'a mut dyn Reflect, &[LuaTableKey])> {
        Ok((self, &[]))
    }

    fn create_sibling(&self, path: Vec<LuaTableKey>) -> mlua::Result<Self::RefType> {        
        Err(lua_error!("operation is not supported, because Lua scripts are not supposed to have fields of non-self-owned types."))
    }
}

impl Reflect for ScriptObject {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let def = self.def.clone();
        let fields = def
            .metadata
            .fields
            .iter()
            .enumerate()
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<ScriptObject>(),
                name: it.name.as_str(),
                display_name: it.name.as_str(),
                description: it.name.as_str(),
                type_name: match it.ty {
                    ScriptFieldValueType::Number => "f32",
                    ScriptFieldValueType::String => "alloc::string::String",
                    ScriptFieldValueType::Bool => "bool",
                    ScriptFieldValueType::Node => "Node",
                    ScriptFieldValueType::Vector3 => "Vector3",
                },
                doc: it.description.unwrap_or(""),
                value: match self.values.get(i).unwrap() {
                    ScriptFieldValue::Number(it) => it,
                    ScriptFieldValue::String(it) => it,
                    ScriptFieldValue::Bool(it) => it,
                    ScriptFieldValue::Node(it) => it,
                    ScriptFieldValue::Vector3(it) => it,
                },
                reflect_value: self
                    .values
                    .get(i)
                    .unwrap()
                    .as_reflect(),
                read_only: false,
                immutable_collection: false,
                min_value: None,
                max_value: None,
                step: None,
                precision: None,
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        let fields = self
            .values
            .iter()
            .map(|it| {
                let it: &dyn Reflect = it.as_reflect();
                it
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        let mut fields = self.values
            .iter_mut()
            .map(|it| {
                let it: &mut dyn Reflect = it.as_reflect_mut();
                it
            })
            .collect::<Vec<_>>();
        func(&mut fields)
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        let def = self.def.clone();
        let value = self
            .values
            .get(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &dyn Reflect = it.as_reflect();
            x
        }))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        let def = self.def.clone();
        let value = self
            .values
            .get_mut(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &mut dyn Reflect = it.as_reflect_mut();
            x
        }))
    }
}
