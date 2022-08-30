use crate::{Span, SpanDebug};
use std::{error::Error, fmt};

pub type LexResult<T> = Result<T, LexError>;

pub struct LexError {
    pub type_: ErrorType,
    pub span: SpanDebug,
}

impl LexError {
    pub fn new(type_: ErrorType, span: &Span, script: &str) -> Self {
        LexError {
            type_,
            span: SpanDebug::from_span(span, script),
        }
    }
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

    #[error("The given script is empty.")]
    EmptyScript,

    #[error("Only found a single colon, expected two. ('::')")]
    SingleColon,
}

/// Creates a [`LexError`] for when trying to tokenize an empty script.
///
/// This is here because [`SpanDebug::from_span`] will return an error for an empty screen.
pub(crate) fn empty_script_error() -> LexError {
    LexError {
        type_: ErrorType::EmptyScript,
        span: SpanDebug {
            start_line: 0,
            end_line: 0,
            text: String::new(),
        },
    }
}
