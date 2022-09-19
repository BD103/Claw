use super::create_statement;
use crate::ast::{Declaration, Statement};
use chumsky::prelude::*;

/// Creates a function declaration parser.
pub fn create_func() -> impl Parser<char, Declaration, Error = Simple<char>> {
    fn create_base_func() -> impl Parser<char, (String, Vec<Statement>), Error = Simple<char>> {
        text::keyword("fn")
            .ignore_then(text::ident().padded())
            .then_ignore(empty().delimited_by(just('('), just(')')))
            .then(
                create_statement()
                    .padded()
                    .repeated()
                    .delimited_by(just('{').padded(), just('}').padded()),
            )
    }

    let event_func = just('@')
        .ignore_then(text::ident().padded())
        .then(create_base_func());

    event_func
        .map(|(event, (name, body))| Declaration::Func {
            name,
            events: vec![event],
            body,
        })
        .or(create_base_func().map(|(name, body)| Declaration::Func {
            name,
            events: Vec::new(),
            body,
        }))
}

/// Creates an enum declaration parser.
pub fn create_declare() -> impl Parser<char, Declaration, Error = Simple<char>> {
    text::keyword("declare")
        .ignore_then(text::ident().padded())
        .then(
            text::ident()
                .padded()
                .separated_by(just(','))
                .allow_trailing()
                .delimited_by(just('{').padded(), just('}').padded()),
        )
        .map(|(kind, items)| Declaration::Declare { kind, items })
}

/// Creates a sprite declaration parser.
pub fn create_sprite() -> impl Parser<char, Declaration, Error = Simple<char>> {
    text::keyword("sprite")
        .ignore_then(text::ident().padded())
        .then(
            create_func()
                .padded()
                .or(create_declare().padded())
                .repeated()
                .delimited_by(just('{').padded(), just('}').padded()),
        )
        .map(|(name, declarations)| Declaration::Sprite { name, declarations })
}
