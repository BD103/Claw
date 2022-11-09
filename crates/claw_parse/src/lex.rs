use claw_middle::Span;
use logos::Logos;

pub fn lex<'source>(source: &'source str) -> Tokenizer<'source> {
    Tokenizer::new(source)
}

pub struct Tokenizer<'source> {
    inner: logos::Lexer<'source, TokenKind>,
}

impl<'source> Tokenizer<'source> {
    fn new(source: &'source str) -> Self {
        Tokenizer {
            inner: TokenKind::lexer(source),
        }
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Token {
            kind: self.inner.next()?,
            span: self.inner.span(),
        })
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq)]
#[non_exhaustive]
pub enum TokenKind {
    #[error]
    Error,
}
