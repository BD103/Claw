use crate::ast::Expression;
use chumsky::prelude::*;

pub fn create_expression() -> impl Parser<char, Expression, Error = Simple<char>> {
    let number = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Expression::Number);

    let text = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Expression::Text);

    let boolean = text::keyword("true")
        .ignored()
        .map(|_| Expression::Boolean(true))
        .or(text::keyword("false")
            .ignored()
            .map(|_| Expression::Boolean(false)));

    number.or(text).or(boolean)
}

pub fn create_call() -> impl Parser<char, Expression, Error = Simple<char>> {
    recursive(|call| {
        text::ident()
            .padded()
            .then_ignore(just("::"))
            .then(text::ident().padded())
            .then(
                create_expression()
                    .or(call)
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .delimited_by(just('(').padded(), just(')').padded()),
            )
            .map(|((module, name), args)| Expression::Call { module, name, args })
    })
}
