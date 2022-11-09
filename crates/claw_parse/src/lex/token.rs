use claw_middle::Span;

pub use crate::token;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[non_exhaustive]
pub enum TokenKind {
    EOF,
}

#[macro_export]
macro_rules! token {
    (EOF) => {
        $crate::lex::token::TokenKind::EOF
    };
}
