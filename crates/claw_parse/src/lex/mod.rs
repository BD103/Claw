mod token;
mod values;

use self::values::{create_bool, create_ident, create_number, create_text};
pub use self::{
    token::*,
    token::{Token, TokenKind},
};

use crate::error::LexError;
use chumsky::prelude::*;

pub fn create_single() -> impl Parser<char, Token, Error = LexError> {
    choice((
        // Values
        create_ident(),
        create_text(),
        create_number(),
        create_bool(),
        // Delimiters
        just('(').to(TokenKind::ParenOpen),
        just(')').to(TokenKind::ParenClose),
        just('{').to(TokenKind::BracketOpen),
        just('}').to(TokenKind::BracketClose),
        // Separators
        just("::").to(TokenKind::DoubleColon),
        just(':').to(TokenKind::Colon),
        just(';').to(TokenKind::Semi),
        just(',').to(TokenKind::Comma),
        // Prefixes
        just('@').to(TokenKind::AtSign),
        just('$').to(TokenKind::EnumSign),
        // Comparison
        just("==").to(TokenKind::Eq),
        just("!=").to(TokenKind::NotEq),
        just("&&").to(TokenKind::And),
        just("||").to(TokenKind::Or),
    ))
    .map_with_span(|kind, span| Token { kind, span })
}

// TODO: Figure out how to ignore everything without all the ignores
pub fn create_comment() -> impl Parser<char, (), Error = LexError> {
    just("//")
        .ignore_then(take_until(choice((just('\n').ignored(), end().ignored()))))
        .ignored()
}

pub fn create_lexer() -> impl Parser<char, Vec<Token>, Error = LexError> {
    choice((create_single().map(Some), create_comment().to(None)))
        .padded()
        .repeated()
        .map(|x| x.into_iter().filter_map(|x| x).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    // create_lexer is tested in integration tests

    #[test]
    fn single() {
        let input = [
            ("(", TokenKind::ParenOpen),
            (")", TokenKind::ParenClose),
            ("{", TokenKind::BracketOpen),
            ("}", TokenKind::BracketClose),
            ("::", TokenKind::DoubleColon),
            (":", TokenKind::Colon),
            (";", TokenKind::Semi),
            (",", TokenKind::Comma),
            ("@", TokenKind::AtSign),
            ("$", TokenKind::EnumSign),
            ("==", TokenKind::Eq),
            ("!=", TokenKind::NotEq),
            ("&&", TokenKind::And),
            ("||", TokenKind::Or),
        ];
        let parser = create_single();

        input.into_iter()
            .for_each(|(s, kind)| {
                let output = parser.parse(s);

                assert!(output.is_ok());

                let output = output.unwrap();

                assert_eq!(output.kind, kind);
                assert_eq!(output.span, 0..s.len());
            });
    }

    #[test]
    fn comment() {
        let input = [
            "// Single line comment",
            "// // // Woah comments",
            "//\n// Another comment",
            "// \n// newlines woooo",
            "// perfectly alined \n",
            "/// as all things sho-",
            "// uld be lol ////////",
        ];
        let parser = create_comment();

        input.into_iter().for_each(|x| {
            let output = parser.parse(x);
            assert_eq!(output, Ok(()));
        });
    }
}
