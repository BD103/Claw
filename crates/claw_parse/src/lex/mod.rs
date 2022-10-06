mod token;
mod values;

pub use self::token::{Token, TokenKind, *};
use self::values::{create_bool, create_ident, create_number, create_text};

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

// `Clone` so it can be used in padded_by.
pub fn create_comment() -> impl Parser<char, (), Error = LexError> + Clone {
    just("//")
        .then(take_until(just('\n')))
        .ignored()
}

pub fn create_lexer() -> impl Parser<char, Vec<Token>, Error = LexError> {
    create_single()
        .padded_by(create_comment().padded().repeated())
        .padded()
        .repeated()
        .then_ignore(end())
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

        input.into_iter().for_each(|(s, kind)| {
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
