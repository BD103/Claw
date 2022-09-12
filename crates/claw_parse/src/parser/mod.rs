use chumsky::prelude::*;
use crate::{AST, ParseError};

pub fn create_parser() -> impl Parser<char, AST, Error = ParseError> {
    empty()
        .then_ignore(end())
}
