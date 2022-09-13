pub type AST = Vec<Declaration>;

#[derive(Clone, Debug)]
pub enum Declaration {
    Func {
        name: String,
        events: Vec<String>,
        body: Vec<Statement>,
    },
    Declare {
        kind: String,
        items: Vec<String>,
    },
    Sprite {
        name: String,
        declarations: Vec<Declaration>,
    },
}

#[derive(Clone, Debug)]
pub enum Statement {
    // Inline Expression::Call into this?
    Call {
        module: String,
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Call {
        module: String,
        name: String,
        args: Vec<Expression>,
    },
    // Keep number in string form to preserve precision
    Number(String),
    Text(String),
    Boolean(bool),
}
