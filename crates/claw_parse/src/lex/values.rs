use super::TokenKind;
use crate::error::LexError;
use chumsky::prelude::*;

pub fn create_ident() -> impl Parser<char, TokenKind, Error = LexError> {
    text::ident().map(TokenKind::Ident)
}

pub fn create_text() -> impl Parser<char, TokenKind, Error = LexError> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(TokenKind::Text)
}

pub fn create_number() -> impl Parser<char, TokenKind, Error = LexError> {
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>();

    // Supports negatives
    choice((
        just('-').then(num.clone()).map(|(_, mut x)| {
            x.insert(0, '-');
            TokenKind::Number(x)
        }),
        num.map(TokenKind::Number),
    ))
}

pub fn create_bool() -> impl Parser<char, TokenKind, Error = LexError> {
    text::keyword("true")
        .ignored()
        .to(TokenKind::Boolean(true))
        .or(text::keyword("false")
            .ignored()
            .to(TokenKind::Boolean(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident() {
        let input = [
            "hi",
            "hiThere",
            "hi_there",
            "_hiThErE",
            "h1Th3r3",
        ];
        // Invalid idents
        let input_err = [
            "5ithere",
            "-hello",
            "🦀",
        ];

        let parser = create_ident();

        input.into_iter()
            .for_each(|x| {
                let output = parser.parse(x);
                assert_eq!(output, Ok(TokenKind::Ident(x.into())));
            });
        
        input_err.into_iter()
            .for_each(|x| {
                let output = parser.parse(x);
                assert!(output.is_err());
            });
        
    }
}
