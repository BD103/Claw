use claw_stdlib::{OpCode, Type};

pub struct Func {
    pub name: String,
}

pub enum Expr {
    Call {
        opcode: OpCode,
        args: Vec<Expr>,
    },
    Number(String),
    Text(String),
    Boolean(bool),
}

impl Expr {
    pub fn returns(&self) -> Option<Type> {
        match self {
            Self::Call { opcode, .. } => opcode.returns(),
            Self::Number(_) => Some(Type::Number),
            Self::Text(_) => Some(Type::Text),
            Self::Boolean(_) => Some(Type::Boolean),
        }
    }
}
