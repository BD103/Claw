mod core;
mod token;

pub use self::{
    core::Lexer,
    token::{Token, TokenKind},
};

pub fn tokenize(script: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(script);

    std::iter::from_fn(move || {
        if lexer.eof() {
            None
        } else {
            Some(lexer.tokenize_one())
        }
    })
}
