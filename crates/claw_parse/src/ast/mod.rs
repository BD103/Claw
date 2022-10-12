//! The abstract-syntax tree used to represent Claw code.
//! 
//! This is generated from a list of [`Token`](crate::lex::Token)s in the [`lex`](crate::lex)er. This conversion process is stored within the [`parse`](crate::parse) module.

pub mod atom;

#[cfg(test)]
mod tests;

/// Represents a span of text.
/// 
/// This is a [`Range`](std::ops::Range) representing where a piece of text starts and stops within the source file.
pub type Span = std::ops::Range<usize>;

/// A declaration within the code.
/// 
/// This is the outer-most portion of the code, and includes function, enum, and sprite definitions.
#[derive(Clone, Debug)]
pub enum Declaration {
    /// A function definition.
    Func {
        name: atom::Ident,
        decorators: Vec<()>,
        body: atom::Body,
        span: Span,
    },
    /// An enum definition.
    Declare {
        kind: atom::Ident,
        items: Vec<DeclareItem>,
        span: Span,
    },
    /// A sprite definition.
    Sprite {
        name: atom::Ident,
        declarations: Vec<Declaration>,
        span: Span,
    },
}

/// An expression within a body of code.
/// 
/// This is the inner-most portion of the code. It represents all kinds of values and function calls.
#[derive(Clone, Debug)]
pub enum Expr {
    /// A function call.
    Call {
        module: atom::Ident,
        name: atom::Ident,
        args: Vec<Expr>,
        span: Span,
    },
    /// A number, whether decimal or integer.
    Number(String, Span),
    /// A string of text.
    Text(String, Span),
    /// A boolean of true or false.
    Boolean(bool, Span),
}

/// An enum item, used within [`Declaration::Declare`].
/// 
/// The reason this is a custom `struct` and not just a list of [`Ident`](atom::Ident)s is so you can set a default value for certain kinds of declarations.
/// 
/// ```claw
/// declare Var {
///     score = 0,
///     time_left = 3999.2,
/// }
/// 
/// declare Sprite {
///     Idle,
///     Walk1,
///     Walk2,
///     Wave,
/// }
/// ```
#[derive(Clone, Debug)]
pub struct DeclareItem {
    pub name: atom::Ident,
    pub value: Option<Expr>,
    pub span: Span,
}
