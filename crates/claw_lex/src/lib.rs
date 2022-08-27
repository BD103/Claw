mod core;
mod cursor;
mod error;
mod span;
mod token;

pub use self::{
    core::tokenize,
    cursor::Cursor,
    error::{ErrorType, LexError, LexResult},
    span::{Span, SpanDebug},
    token::{Token, TokenType},
};
