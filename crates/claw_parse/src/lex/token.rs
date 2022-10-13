use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenKind {
    // Values
    /// Either an identifier or keyword.
    ///
    /// `fn`, `looks`, etc.
    Ident(String),
    /// A string.
    ///
    /// `"Some text"`
    Text(String),
    /// A number, optional decimal and sign.
    ///
    /// `64`, `-2`, `3.1415`
    Number(String),
    /// True or false.
    ///
    /// `true` or `false`
    Boolean(bool),

    // Delimiters
    /// `(`
    ParenOpen,
    /// `)`
    ParenClose,
    /// `{`
    BracketOpen,
    /// `}`
    BracketClose,

    // Separators
    /// `::`
    DoubleColon,
    /// `:`
    Colon,
    /// `;`
    Semi,
    /// `,`
    Comma,

    // Prefixes
    /// `@`
    AtSign,
    /// `$`
    EnumSign,

    // Comparison
    /// `==`
    Eq,
    /// `!=`
    NotEq,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `<`
    Lt,
    /// `<=`
    Le,
}
