pub mod atom;

#[cfg(test)]
mod tests;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Declaration {
    Func {
        name: atom::Ident,
        decorators: Vec<()>,
        body: atom::Body,
        span: Span,
    },
    Declare {
        kind: atom::Ident,
        items: Vec<DeclareItem>,
        span: Span,
    },
    Sprite {
        name: atom::Ident,
        declarations: Vec<Declaration>,
        span: Span,
    },
}

#[derive(Clone, Debug)]
pub enum Expr {
    Call {
        module: atom::Ident,
        name: atom::Ident,
        args: Vec<Expr>,
        span: Span,
    },
    Number(String, Span),
    Text(String, Span),
    Boolean(bool, Span),
}

#[derive(Clone, Debug)]
pub struct DeclareItem {
    pub name: atom::Ident,
    pub value: Option<Expr>,
    pub span: Span,
}
