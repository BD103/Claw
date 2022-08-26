use crate::Scope;
use claw_lex::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Encountered an invalid token '{0:?}' after token '{1:?}'.")]
    InvalidTokenAfter(Token, Token),

    #[error("Did not find expected token after token '{0:?}'.")]
    NoTokenAfter(Token),

    #[error("Found invalid declaration type '{0}'.")]
    InvalidDeclarationType(String),

    #[error("Found invalid declaration in scope '{0}'.")]
    InvalidDeclarationScope(Scope),
}
