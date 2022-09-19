mod declare;
mod expr;
mod statement;

pub use self::{declare::*, expr::*, statement::*};

use crate::{ast::AST, ParseError};
use chumsky::prelude::*;

pub fn create_comment() -> impl Parser<char, (), Error = ParseError> {
    just("//").then(take_until(just('\n'))).padded().ignored()
}

pub fn create_parser() -> impl Parser<char, AST, Error = Simple<char>> {
    create_comment()
        .repeated()
        .ignore_then(
            create_func()
                .padded()
                .or(create_declare().padded())
                .or(create_sprite().padded()),
        )
        .then_ignore(create_comment().repeated())
        .repeated()
        .then_ignore(end())
}
