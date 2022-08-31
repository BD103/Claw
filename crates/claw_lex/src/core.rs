use crate::Token;
use std::str::Chars;

/// A lexer that tokenizes a string into ['Token'](crate::Token)s.
pub struct Lexer<'a> {
    /// An iterator over the [`char`]s in a string.
    chars: Chars<'a>,
    /// The current position of the cursor.
    pos: u32,
}

impl<'a> Lexer<'a> {
    /// Creates a new ['Lexer'].
    pub fn new(script: &'a str) -> Self {
        Lexer {
            chars: script.chars(),
            pos: 0,
        }
    }

    /// Returns the next token.
    pub fn tokenize_one(&mut self) -> Token {
        todo!()
    }

    /// Returns true if the lexer has reached the end of the file.
    pub fn eof(&self) -> bool {
        self.chars.clone().as_str().is_empty()
    }
}
