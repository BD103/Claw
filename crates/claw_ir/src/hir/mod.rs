use crate::builtin::OpCode;

pub struct Func {
    pub name: String,
}

pub enum Expr {
    Call {
        opcode: OpCode,
        args: Vec<Expr>,
    }
}
