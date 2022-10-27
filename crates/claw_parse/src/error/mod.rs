use ariadne::{Report, ReportKind};
use chumsky::error::Simple;

pub type LexError = Simple<char>;
pub type ParseError = ();

pub fn generate_token_report(_errors: Vec<Simple<char>>) -> Report {
    Report::build(ReportKind::Error, (), 0).finish()
}
