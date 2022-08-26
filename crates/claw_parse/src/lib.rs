mod context;
mod errors;
mod parsers;
mod target;
pub mod utils;

pub use self::{
    context::{Context, Scope},
    errors::ParseError,
    parsers::*,
    target::Stage,
};

use claw_lex::Token;
use std::iter::Peekable;

pub type ParseResult<T> = Result<T, ParseError>;

fn parse_single<I: Iterator<Item = Token>>(
    it: &mut Peekable<I>,
    stage: &mut Stage,
    ctx: &mut Context,
) -> ParseResult<()> {
    // May be superfluous, but it prevents token being passed into function
    if let Some(token) = it.next() {
        match token {
            Token::IdentOrKeyword(value) => match <String as AsRef<str>>::as_ref(&value) {
                "declare" => match parse_declaration(it, stage, ctx) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                "sprite" => match parse_sprite(it, stage, ctx) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                _ => {}
            },
            _ => {
                // Will be removed later when all tokens are handled
            }
        }
    }

    Ok(())
}

pub fn parse(tokens: Vec<Token>) -> ParseResult<Stage> {
    let mut it = tokens.into_iter().peekable();
    let mut stage = Stage::default();
    let mut ctx = Context::default();

    // Repeat while tokens are left
    while let Some(&_) = it.peek() {
        match parse_single(&mut it, &mut stage, &mut ctx) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(stage)
}
