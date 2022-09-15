use std::convert::{TryFrom, TryInto};

use crate::{HIR, hir};

#[derive(Default, Debug)]
pub struct LIR {
    pub declarations: Declarations,
    pub functions: Vec<Func>,
    pub sprites: Vec<Sprite>,
}

impl TryFrom<HIR> for LIR {
    type Error = &'static str;

    fn try_from(mut value: HIR) -> Result<Self, Self::Error> {
        let mut result = LIR::default();

        // Create declarations
        for (k, mut v) in value.declarations.drain().take(1) {
            if k == "Var" {
                result.declarations.var.append(&mut v);
            } else {
                return Err("Declared non-Var type.");
            }
        }

        for sprite in value.sprites {
            result.sprites.push(sprite.try_into()?);
        }

        for func in value.functions {
            result.functions.push(func.try_into()?);
        }

        Ok(result)
    }
}

#[derive(Default, Debug)]
pub struct Declarations {
    pub var: Vec<String>,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub declarations: Declarations,
    pub functions: Vec<Func>,
}

impl TryFrom<hir::Sprite> for Sprite {
    type Error = &'static str;

    fn try_from(mut value: hir::Sprite) -> Result<Self, Self::Error> {
        let mut declarations = Declarations::default();
        let mut functions = Vec::new();

        for (k, mut v) in value.declarations.drain().take(1) {
            if k == "Var" {
                declarations.var.append(&mut v);
            } else {
                return Err("Declared non-Var type.");
            }
        }

        for func in value.functions {
            functions.push(func.try_into()?);
        }

        Ok(Sprite {
            name: value.name,
            declarations: declarations,
            functions: functions,
        })
    }
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub events: Vec<String>,
    pub body: Vec<Expression>,
}

impl TryFrom<hir::Func> for Func {
    type Error = &'static str;

    fn try_from(value: hir::Func) -> Result<Self, Self::Error> {
        let mut result = Func {
            name: value.name,
            events: value.events,
            body: Vec::new(),
        };

        for expr in value.body {
            result.body.push(expr.try_into()?);
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub enum Expression {
    Call {
        opcode: String,
        args: Vec<Expression>,
    },
    Number(String),
    Text(String),
    Boolean(bool),
}

impl TryFrom<claw_parse::ast::Statement> for Expression {
    type Error = &'static str;

    fn try_from(value: claw_parse::ast::Statement) -> Result<Self, Self::Error> {
        use claw_parse::ast::Statement as AstStatement;

        Ok(match value {
            AstStatement::Call { module, name, args } => {
                Expression::Call {
                    opcode: claw_stdlib::get_opcode(module, name).ok_or("Could not find opcode")?.to_string(),
                    args: {
                        let mut res = Vec::new();

                        for e in args {
                            res.push(e.try_into()?);
                        }

                        res
                    }
                }
            }
        })
    }
}

impl TryFrom<claw_parse::ast::Expression> for Expression {
    type Error = &'static str;

    fn try_from(value: claw_parse::ast::Expression) -> Result<Self, Self::Error> {
        use claw_parse::ast::Expression as AstExpr;

        Ok(match value {
            AstExpr::Call { module, name, args } => {
                Expression::Call {
                    opcode: claw_stdlib::get_opcode(module, name).ok_or("Could not find opcode")?.to_string(),
                    args: {
                        let mut res = Vec::new();

                        for e in args {
                            res.push(e.try_into()?);
                        }

                        res
                    }
                }
            }
            AstExpr::Number(x) => Expression::Number(x),
            AstExpr::Text(x) => Expression::Text(x),
            AstExpr::Boolean(x) => Expression::Boolean(x),
        })
    }
}