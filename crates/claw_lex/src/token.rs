/// A parsed token containing the kind and size.
#[derive(Debug)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,

    /// The length in characters of the token.
    pub len: u32,
}

impl Token {
    /// Creates a new [`Token`].
    #[inline]
    pub(crate) fn new(kind: TokenKind, len: u32) -> Self {
        Token { kind, len }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    /// An identifier or keyword.
    Ident,
    /// A literal, e.g. `"Meow"`, `3.14`, `true`.
    Literal { kind: LiteralKind },

    /// "("
    ParenOpen,
    /// ")"
    ParenClose,
    /// "{"
    BraceOpen,
    /// "}"
    BraceClose,

    /// "@"
    At,
    /// "$"
    Dollar,
    /// ","
    Coma,
    /// ":"
    Colon,
    /// ";"
    Semi,

    /// Invisible characters that are ignored by the lexer, e.g. a newline.
    Whitespace,
    /// An unknown token not expected by the lexer.
    Unknown,
}

#[derive(Debug)]
pub enum LiteralKind {
    /// Represents a whole number.
    Integer,

    /// Represents a decimal number.
    Decimal,

    /// Represents a string of characters.
    Text { terminated: bool },
}
