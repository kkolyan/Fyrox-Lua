use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct GlobalName(pub String);

pub struct Imports {
    wildcard: Vec<String>,
    crate_name: String,
    full: Vec<String>,
}

pub struct Identifier {
    pub(crate) ident: String,

    /// name of mod, to resolve the qualified name against its usings and declarations
    pub(crate) context_mod: String,
}

impl Display for GlobalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Default)]
pub struct EngineApi {
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub constants: Vec<Const>,
}

struct Class {
    name: GlobalName,
    properties: Vec<Property>,
    methods: Vec<Method>,
}

pub struct Function {
    pub name: String,
	pub def: String,
}

struct Property {
    name: String,
}

struct Const {
    name: GlobalName,
    value: String,
}

struct Method {
    name: String,
}

pub(crate) struct ModCtx {
    pub(crate) full_path: String,
    pub(crate) symbols: HashMap<String, GlobalName>,
}
impl ModCtx {
    pub(crate) fn new(full_path: String) -> Self {
        Self {
            full_path,
            symbols: Default::default(),
        }
    }
}

pub enum Alias {
    ///
    Type(Identifier),
    Use(String),
}

#[derive(Default)]
pub struct Parser {
    pub(crate) result: EngineApi,
    /// first entry is crate name
    pub(crate) mod_stack: Vec<ModCtx>,
    pub(crate) global_wildcards: Vec<(String, String)>,
    pub(crate) global_aliases: HashMap<String, Alias>,
	pub(crate) known_objects: Vec<GlobalName>,
	pub(crate) methods: Vec<(Identifier, Method)>,
}

impl Parser {
    pub(crate) fn push_mod(&mut self, name: impl Display) {
        let full_path = format!("{}::{}", &self.mod_stack.last().unwrap().full_path, name);
        self.mod_stack.push(ModCtx::new(full_path));
    }

    pub(crate) fn pop_mod(&mut self) {
        self.mod_stack.pop().unwrap();
    }
}
