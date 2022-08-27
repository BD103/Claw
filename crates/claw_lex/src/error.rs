use crate::SpanDebug;
use std::{error::Error, fmt};

pub type LexResult<T> = Result<T, LexError>;

pub struct LexError {
    pub type_: ErrorType,
    pub span: SpanDebug,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Could not lex given script.")?;

        if self.span.is_single_line() {
            writeln!(f, "\nInvalid code on line {}.", self.span.start_line)?;
        } else {
            writeln!(
                f,
                "\nInvalid code on line {} through {}.",
                self.span.start_line, self.span.end_line
            )?;
        }

        writeln!(f, "\n{}", self.span.text)?;

        write!(f, "\n{}", self.type_)
    }
}

impl fmt::Debug for LexError {
    /// Formats the same as [`fmt::Display`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl Error for LexError {}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorType {
    #[error("Unknown token {0:?} found.")]
    UnknownToken(char),
}
