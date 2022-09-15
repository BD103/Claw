// TODO: Use hard error types instead of str for TryFrom.

use claw_parse::ast;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

/// The intermediate representation of a Claw script.
#[derive(Default, Debug, Clone)]
pub struct HIR {
    pub declarations: HashMap<String, Vec<String>>,
    pub functions: Vec<Func>,
    pub sprites: Vec<Sprite>,
}

impl TryFrom<ast::AST> for HIR {
    type Error = &'static str;

    fn try_from(value: ast::AST) -> Result<Self, Self::Error> {
        let mut result = HIR::default();

        for i in value {
            match i {
                ast::Declaration::Declare { kind, mut items } => {
                    if !result.declarations.contains_key(&kind) {
                        result.declarations.insert(kind, items);
                    } else {
                        result
                            .declarations
                            .get_mut(&kind)
                            .unwrap()
                            .append(&mut items);
                    }
                }
                ast::Declaration::Func { .. } => result.functions.push(Func::try_from(i)?),
                ast::Declaration::Sprite { .. } => result.sprites.push(Sprite::try_from(i)?),
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub name: String,
    pub declarations: HashMap<String, Vec<String>>,
    pub functions: Vec<Func>,
}

impl TryFrom<ast::Declaration> for Sprite {
    type Error = &'static str;

    fn try_from(value: ast::Declaration) -> Result<Self, Self::Error> {
        if let ast::Declaration::Sprite { name, declarations } = value {
            // TODO: this code is messy, rewrite later
            let (declarations, functions) =
                declarations
                    .into_iter()
                    .fold((HashMap::new(), Vec::new()), |mut acc, x| {
                        match x {
                            ast::Declaration::Declare { kind, mut items } => {
                                if !acc.0.contains_key(&kind) {
                                    acc.0.insert(kind, items);
                                } else {
                                    acc.0.get_mut(&kind).unwrap().append(&mut items);
                                }
                            }
                            ast::Declaration::Func { .. } => acc.1.push(x.try_into().unwrap()),
                            _ => {} // Ignore non-func / sprite
                        }

                        acc
                    });

            Ok(Sprite {
                name,
                declarations,
                functions,
            })
        } else {
            Err("Please convert with the Declaration::Sprite type.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub events: Vec<String>,
    pub body: Vec<ast::Statement>,
}

impl TryFrom<ast::Declaration> for Func {
    type Error = &'static str;

    fn try_from(value: ast::Declaration) -> Result<Self, Self::Error> {
        if let ast::Declaration::Func { name, events, body } = value {
            Ok(Func { name, events, body })
        } else {
            Err("Please convert with the Declaration::Func type.")
        }
    }
}
