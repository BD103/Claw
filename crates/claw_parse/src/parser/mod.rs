mod declare;
mod expr;
mod statement;

pub use self::{declare::*, expr::*, statement::*};

use crate::ast::*;
use chumsky::prelude::*;

pub fn create_parser() -> impl Parser<char, Vec<Declaration>, Error = Simple<char>> {
    let comment = just("//").then(take_until(just('\n'))).padded().ignored();

    create_func()
        .padded()
        .or(create_declare().padded())
        .or(create_sprite().padded())
        .padded_by(comment.repeated())
        .repeated()
        .then_ignore(end())
}
