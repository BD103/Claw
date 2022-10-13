use ariadne::{Report, ReportKind};
use chumsky::error::Simple;

use crate::lex::Token;

pub type LexError = Simple<char>;
pub type ParseError = Simple<Token>;

pub fn generate_token_report(_errors: Vec<Simple<char>>) -> Report {
    Report::build(ReportKind::Error, (), 0)
        .finish()
}
