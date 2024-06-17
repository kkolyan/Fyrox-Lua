use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use fyrox::{
    core::reflect::prelude::*, core::type_traits::prelude::*,
    core::visitor::prelude::*
};
use std::path::Path;
use std::sync::Arc;
use fyrox::script::{BaseScript, ScriptTrait};
use std::fs;
use std::io::BufRead;
use fyrox::core::algebra::Vector3;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;

mod metadata;
mod reflect;

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

#[derive(Debug)]
pub struct ScriptDefinition {
    pub metadata: ScriptMetadata,
    pub assembly_name: &'static str,
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
pub struct ScriptField {
    pub name: String,
    pub ty: ScriptFieldValueType,
    pub description: Option<&'static str>,
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

impl Debug for ScriptFieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScriptFieldValue")
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

impl ScriptTrait for LuaScript {
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
