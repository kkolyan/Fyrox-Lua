use fyrox::core::reflect::{FieldInfo, Reflect};
use std::any::{Any, TypeId};
use crate::script::{LuaScript, ScriptFieldValue, ScriptFieldValueType};

impl Reflect for ScriptFieldValue {
    fn source_path() -> &'static str where Self: Sized {
        file!()
    }

    fn type_name(&self) -> &'static str {
        match self {
            ScriptFieldValue::Number(_) => "number",
            ScriptFieldValue::String(_) => "string",
            ScriptFieldValue::Bool(_) => "bool",
            ScriptFieldValue::Node(_) => "Node",
            ScriptFieldValue::Vector3(_) => "Vector3",
        }
    }

    fn doc(&self) -> &'static str {
        ""
    }

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        match self {
            ScriptFieldValue::Number(it) => it.fields_info(func),
            ScriptFieldValue::String(it) => it.fields_info(func),
            ScriptFieldValue::Bool(it) => it.fields_info(func),
            ScriptFieldValue::Node(it) => it.fields_info(func),
            ScriptFieldValue::Vector3(it) => it.fields_info(func),
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        match *self {
            ScriptFieldValue::Number(it) => Box::new(it),
            ScriptFieldValue::String(it) => Box::new(it),
            ScriptFieldValue::Bool(it) => Box::new(it),
            ScriptFieldValue::Node(it) => Box::new(it),
            ScriptFieldValue::Vector3(it) => Box::new(it),
        }
    }

    fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
        match self {
            ScriptFieldValue::Number(it) => func(it),
            ScriptFieldValue::String(it) => func(it),
            ScriptFieldValue::Bool(it) => func(it),
            ScriptFieldValue::Node(it) => func(it),
            ScriptFieldValue::Vector3(it) => func(it),
        }
    }

    fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
        match self {
            ScriptFieldValue::Number(it) => func(it),
            ScriptFieldValue::String(it) => func(it),
            ScriptFieldValue::Bool(it) => func(it),
            ScriptFieldValue::Node(it) => func(it),
            ScriptFieldValue::Vector3(it) => func(it),
        }
    }

    fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
        match self {
            ScriptFieldValue::Number(it) => func(it),
            ScriptFieldValue::String(it) => func(it),
            ScriptFieldValue::Bool(it) => func(it),
            ScriptFieldValue::Node(it) => func(it),
            ScriptFieldValue::Vector3(it) => func(it),
        }
    }

    fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
        match self {
            ScriptFieldValue::Number(it) => func(it),
            ScriptFieldValue::String(it) => func(it),
            ScriptFieldValue::Bool(it) => func(it),
            ScriptFieldValue::Node(it) => func(it),
            ScriptFieldValue::Vector3(it) => func(it),
        }
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        match self {
            ScriptFieldValue::Number(it) => it.set(value),
            ScriptFieldValue::String(it) => it.set(value),
            ScriptFieldValue::Bool(it) => it.set(value),
            ScriptFieldValue::Node(it) => it.set(value),
            ScriptFieldValue::Vector3(it) => it.set(value),
        }
    }

    fn assembly_name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn type_assembly_name() -> &'static str where Self: Sized {
        env!("CARGO_PKG_NAME")
    }
}

impl Reflect for LuaScript {
    fn source_path() -> &'static str where Self: Sized {
        panic!("is not expected to be called")
    }

    fn type_name(&self) -> &'static str {
        self.def.metadata.class
    }

    fn doc(&self) -> &'static str {
        ""
    }

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let fields = self.def.metadata.fields.iter()
            .enumerate()
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<LuaScript>(),
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
                reflect_value: self.values.get(i).unwrap(),
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

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
        func(self)
    }

    fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
        func(self)
    }

    fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
        func(self)
    }

    fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
        func(self)
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        let this = std::mem::replace(self, value.take()?);
        Ok(Box::new(this))
    }

    fn assembly_name(&self) -> &'static str {
        self.def.assembly_name
    }

    fn type_assembly_name() -> &'static str where Self: Sized {
        panic!("is not expected to be called")
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        let fields = self.values.iter()
            .map(|it| {
                let it: &dyn Reflect = it;
                it
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        let mut fields = self.values.iter_mut()
            .map(|it| {
                let it: &mut dyn Reflect = it;
                it
            })
            .collect::<Vec<_>>();
        func(&mut fields)
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        let value = self.values.get(self.def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &dyn Reflect = it;
            x
        }))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        let value = self.values.get_mut(self.def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &mut dyn Reflect = it;
            x
        }))
    }
}
