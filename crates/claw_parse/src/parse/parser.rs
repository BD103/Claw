use crate::lex::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        if self.pos + offset < self.tokens.len() {
            Some(&self.tokens[self.pos + offset])
        } else {
            None
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
