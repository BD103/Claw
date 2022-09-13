use chumsky::prelude::*;
use std::time::Instant;

#[derive(Clone, Debug)]
pub enum Declaration {
    Func {
        name: String,
        events: Vec<String>,
        body: Vec<Statement>,
    },
    Declare {
        kind: String,
        items: Vec<String>,
    },
    Sprite {
        name: String,
        declarations: Vec<Declaration>,
    },
}

#[derive(Clone, Debug)]
pub enum Statement {
    // Inline Expression::Call into this?
    Call {
        module: String,
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Call {
        module: String,
        name: String,
        args: Vec<Expression>,
    },
    Number(String),
    Text(String),
    Boolean(bool),
}

fn create_func() -> impl Parser<char, Declaration, Error = Simple<char>> {
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

fn create_declare() -> impl Parser<char, Declaration, Error = Simple<char>> {
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

fn create_sprite() -> impl Parser<char, Declaration, Error = Simple<char>> {
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

fn create_statement() -> impl Parser<char, Statement, Error = Simple<char>> {
    create_call()
        .then_ignore(just(';').padded())
        .map(|call| match call {
            Expression::Call { module, name, args } => Statement::Call { module, name, args },
            _ => {
                unreachable!("Tried to convert non-call to a statement, please file a bug report!")
            }
        })
}

fn create_expression() -> impl Parser<char, Expression, Error = Simple<char>> {
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

fn create_call() -> impl Parser<char, Expression, Error = Simple<char>> {
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

fn main() {
    let time_start = Instant::now();

    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let time_file_read = time_start.elapsed();

    let parsed = create_parser().parse(src.clone());
    let time_parse = time_start.elapsed();

    match parsed {
        Ok(ast) => println!("{:#?}", ast),
        Err(parse_errs) => {
            use ariadne::{Report, ReportKind, Source, Label};

            let err = parse_errs.first().unwrap();

            Report::build(ReportKind::Error, (), err.span().start)
                .with_label(Label::new(err.span()).with_message(format!("{}", err)))
                .finish()
                .print(Source::from(src))
                .unwrap();
        },
    }

    println!(
        "File Read Time: {}\nParse Time: {}",
        time_file_read.as_secs_f32(),
        time_parse.as_secs_f32()
    );
}
