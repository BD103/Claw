use claw_parse::ast;
use std::collections::HashMap;

/// The intermediate representation of a Claw script.
#[derive(Debug, Clone)]
pub struct HIR {
    pub declarations: HashMap<String, Vec<String>>,
    pub functions: Vec<Func>,
    pub sprites: Vec<Sprite>,
}

impl Default for HIR {
    fn default() -> Self {
        HIR {
            declarations: HashMap::new(),
            functions: Vec::new(),
            sprites: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub name: String,
    pub declarations: HashMap<String, Vec<String>>,
    pub functions: Vec<Func>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub events: Vec<String>,
    pub body: Vec<ast::Statement>,
}
