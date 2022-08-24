use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

// TODO: Find better way then embedding line number in each error message.

/// Represents all error types that may be returned when lexing a script.
///
/// Due to the nature of [`thiserror`], it is recommended to view source to see
/// the debug messages for each error. All errors contain a [`usize`], which is
/// the line number for which the error occured. This is created from
/// [`Cursor::line`](super::cursor::Cursor::line).
#[derive(Error, Debug)]
pub enum LexError {
    #[error("Could not find closing quotation mark for string. Line: '{0}'.")]
    NoClosingQuotationMark(usize),

    /// The [`char`] is the unknown character.
    #[error("Encountered unknown character {0} while lexing. Line: '{1}'.")]
    UnkownToken(char, usize),

    #[error("Ecountered newline while lexing string. Line: '{0}'.")]
    StringNewline(usize),

    /// The [`ParseIntError`] is what is returned when calling
    /// [`str::parse::<i64>`].
    #[error("Error converting String to integer (i64). '{0}', Line: '{1}'.")]
    IntegerParse(ParseIntError, usize),

    /// The [`ParseFloatError`] is what is returned when calling
    /// [`str::parse::<f64>`].
    #[error("Error converting String to float (f64). '{0}', Line: '{1}'.")]
    FloatParse(ParseFloatError, usize),

    // Will be removed in future releases with variable syntax
    #[error("Found only a single '='. Did you mean '=='? (Note: assigning variables with '=' is not yet supported.) Line: '{0}'.")]
    SingleEq(usize),

    #[error("Found only a single '&'. Did you mean '&&'? Line: '{0}'.")]
    SingleAnd(usize),

    #[error("Found only a single '|'. Did you mean '||'? Line: '{0}'.")]
    SingleOr(usize),

    #[error("Found a triple colon ':::'. Did you mean '::'? Line: '{0}'.")]
    TripleColon(usize),
}
