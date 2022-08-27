use crate::Span;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub type_: TokenType,
    pub span: Span,
}

impl Token {
    #[inline]
    pub fn new(type_: TokenType, span: Span) -> Self {
        Token { type_, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    IdentOrKeyword,
    Text,
    Number,
    Boolean,

    ParanOpen,
    ParanClose,
    BracketOpen,
    BracketClose,

    AtSign,
    EnumSign,
    Comma,
    DoubleColon,
    Semi,

    Whitespace,
    Unknown,
}
