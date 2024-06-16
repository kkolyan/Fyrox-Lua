use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use fyrox::{
    core::{
        type_traits::prelude::*,
    },  core::reflect::prelude::*,
    core::visitor::prelude::*,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene
};
use std::path::Path;
use std::sync::Arc;
use fyrox::script::{BaseScript, ScriptTrait};
use std::fs;
use std::io::BufRead;
use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;

#[derive(Debug, Clone, ComponentProvider)]
pub struct LuaScript {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue>,
}

#[derive(Clone)]
pub enum ScriptFieldValue {
    Number(f64),
    String(String),
    Bool(bool),
    Node(Handle<Node>),
    Vector3(Vector3<f32>),
}

#[derive(Copy, Clone, Debug)]
pub enum ScriptFieldValueType {
    Number,
    String,
    Bool,
    Node,
    Vector3,
}

impl Visit for LuaScript {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        for (i, field) in self.def.metadata.fields.iter().enumerate() {
            self.values[i].visit(&field.name, &mut guard)?;
        }

        Ok(())
    }
}

impl Debug for ScriptFieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScriptFieldValue")
    }
}

impl Visit for ScriptFieldValue {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        match self {
            ScriptFieldValue::Number(it) => it.visit(name, visitor),
            ScriptFieldValue::String(it) => it.visit(name, visitor),
            ScriptFieldValue::Bool(it) => it.visit(name, visitor),
            ScriptFieldValue::Node(it) => it.visit(name, visitor),
            ScriptFieldValue::Vector3(it) => it.visit(name, visitor),
        }
    }
}

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

impl LuaScript {
    pub fn new(def: &Arc<ScriptDefinition>) -> Self {
        Self {
            def: def.clone(),
            values: def.metadata.fields.iter()
                .map(|it| {
                    match it.ty {
                        ScriptFieldValueType::Number => ScriptFieldValue::Number(Default::default()),
                        ScriptFieldValueType::String => ScriptFieldValue::String(Default::default()),
                        ScriptFieldValueType::Bool => ScriptFieldValue::Bool(Default::default()),
                        ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                        ScriptFieldValueType::Vector3 => ScriptFieldValue::Vector3(Default::default()),
                    }
                })
                .collect(),
        }
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
}

impl BaseScript for LuaScript {
    fn clone_box(&self) -> Box<dyn ScriptTrait> {
        Box::new(self.clone())
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn id(&self) -> Uuid {
        self.def.metadata.uuid
    }
}

impl ScriptTrait for LuaScript {

}

#[derive(Debug)]
pub struct ScriptMetadata {
    pub uuid: Uuid,
    pub class: &'static str,
    pub parent_class: Option<String>,
    pub fields: Vec<ScriptField>,
    pub field_name_to_index: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct ScriptDefinition {
    pub metadata: ScriptMetadata,
    pub assembly_name: &'static str,
}

#[derive(Debug)]
pub struct ScriptField {
    pub name: String,
    pub ty: ScriptFieldValueType,
    pub description: Option<&'static str>,
}

impl ScriptMetadata {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let script_source = fs::read(&path).unwrap();
        let path = path.as_ref();
        let file_name = match path.file_name() {
            None => return Err(format!("failed to get name of file {}", &path.to_string_lossy())),
            Some(it) => it.to_string_lossy(),
        };
        Self::parse_source(script_source)
            .and_then(|it| {
                if !file_name.starts_with(format!("{}.", it.class).as_str()) {
                    Err(format!("file name should match class name. class: {}", it.class))
                } else {
                    Ok(it)
                }
            })
    }

    /*
Example:
---@uuid 9596c7ea-8751-423d-839a-5f6c8364223c
---@class Bullet: Script
---@field velocity Vector3
---@field damage number
---@field ttl_sec number
---@field owner Node
     */
    pub fn parse_source(script_source: Vec<u8>) -> Result<ScriptMetadata, String> {
        let mut uuid = None;
        let mut class = None;
        let mut parent_class = None;
        let mut fields = Vec::new();
        for line in script_source.lines() {
            match line {
                Ok(line) => {
                    let annotation_prefix = "---@";
                    if !line.starts_with(annotation_prefix) {
                        break;
                    }
                    let annotation = &line[annotation_prefix.len()..];
                    if let Some((tag, value)) = annotation.split_once(" ") {
                        match tag {
                            "uuid" => {
                                if uuid.is_some() {
                                    return Err(format!("duplicated uuid tag"));
                                }
                                match Uuid::parse_str(value) {
                                    Ok(value) => {
                                        uuid = Some(value);
                                    }
                                    Err(err) => {
                                        return Err(format!("failed to parse UUID from text {:?} due to {}", value, err));
                                    }
                                }
                            }
                            "class" => {
                                if class.is_some() {
                                    return Err(format!("duplicated class tag"));
                                }
                                let mut parts = value.splitn(2, ":");
                                class = Some(parts.next().unwrap().trim().to_string().leak());
                                parent_class = parts.next().map(|it| it.trim().to_string());
                            }
                            "field" => {
                                let mut parts = value.splitn(3, " ");
                                let name = parts.next().unwrap().to_string();
                                let ty = match parts.next() {
                                    None => return Err(format!("failed to parse field type: {}", &annotation)),
                                    Some(it) => match it {
                                        "number" => ScriptFieldValueType::Number,
                                        "string" => ScriptFieldValueType::String,
                                        "bool" => ScriptFieldValueType::Bool,
                                        "Node" => ScriptFieldValueType::Node,
                                        "Vector3" => ScriptFieldValueType::Vector3,
                                        unsupported => return Err(format!("type not supported: {}", unsupported)),
                                    },
                                };
                                let description = parts.next().map(|it| {
                                    let s: &'static str = it.to_string().leak();
                                    s
                                });
                                fields.push(ScriptField { name, ty, description });
                            }
                            unknown_tag => {
                                return Err(format!("unknown tag: {}", unknown_tag));
                            }
                        }
                        continue;
                    }
                    return Err(format!("invalid tag: {}", annotation));
                }
                Err(err) => {
                    return Err(format!("failed to extract lines of text: {}", err));
                }
            }
        }
        if uuid.is_none() {
            return Err("uuid tag is missing".to_string());
        }
        if class.is_none() {
            return Err("class tag is missing".to_string());
        }
        let field_name_to_index = fields.iter().enumerate().map(|(i, v)| (v.name.clone(), i)).collect();
        Ok(ScriptMetadata {
            uuid: uuid.unwrap(),
            class: class.unwrap(),
            parent_class,
            fields,
            field_name_to_index,
        })
    }
}
