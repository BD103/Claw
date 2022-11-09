mod token;

pub use self::token::{Token, TokenKind, token};

pub fn lex<'a>(source: &'a str) -> Tokenizer<'a> {
    Tokenizer::new(source)
}

pub struct Tokenizer<'a> {
    cursor: usize,
    source: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Tokenizer {
            cursor: 0,
            source,
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor > self.source.len() - 1 {
            return Some(Token {
                kind: token!(EOF),
                span: 0..0,
            });
        }

        todo!()
    }
}
