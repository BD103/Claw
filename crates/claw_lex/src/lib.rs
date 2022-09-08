pub use logos::Lexer;

use logos::Logos;

/// A token in a script.
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // a-z with support and numbers and _ after first char.
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    // Any number with optional decimal point and numbers after.
    #[regex(r"[0-9]+(\.[0-9]*)?")]
    NumberLiteral,
    // Any non-newline within quotes.
    #[regex(r#""[^\n]*""#)]
    TextLiteral,

    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,

    #[token("@")]
    At,
    #[token("$")]
    Dollar,
    #[token(",")]
    Comma,
    #[token("::")]
    DoubleColon,
    #[token(";")]
    Semi,

    // Handle errors
    #[error]
    // Skip whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Tokenizes a script into a series of tokens.
pub fn tokenize(script: &str) -> Lexer<Token> {
    Token::lexer(script)
}
