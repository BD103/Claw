use super::create_call;
use crate::ast::{Expression, Statement};
use chumsky::prelude::*;

/// Creates a statement parser.
///
/// In reality, this is just [`create_call`] but then a semicolon (`;`) is required afterwards.
pub fn create_statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    create_call()
        .then_ignore(just(';').padded())
        .map(|call| match call {
            Expression::Call { module, name, args } => Statement::Call { module, name, args },
            _ => {
                unreachable!("Tried to convert non-call to a statement, please file a bug report!")
            }
        })
}
