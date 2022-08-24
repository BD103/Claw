use crate::{utils::ensure_next_token, Context, ParseError, ParseResult, Scope, Stage};
use claw_lex::Token;
use std::iter::Peekable;

#[allow(unused_variables, unreachable_code)]
pub fn parse_declaration<I: Iterator<Item = Token>>(
    it: &mut Peekable<I>,
    stage: &mut Stage,
    ctx: &mut Context,
) -> ParseResult<()> {
    // Declarations cannot be made within a function
    match ctx.scope {
        Scope::Stage | Scope::Sprite(_) => {}
        Scope::StageFn | Scope::SpriteFn(_) => {
            return Err(ParseError::InvalidDeclarationScope(ctx.scope.clone()))
        }
    }

    // After "declare", must be Ident.
    ensure_next_token!(
        it,
        Token::IdentOrKeyword(declare_type),
        Token::IdentOrKeyword("declare".to_string()),
        {
            // Verify '{' after declare_type
            ensure_next_token!(
                it,
                Token::BracketOpen,
                Token::IdentOrKeyword(declare_type.clone()),
                {},
            );

            // Store the previous token for error messages
            let mut last = Token::BracketOpen;

            while let Some(t) = it.next() {
                // Needs to clone for 'last' to work
                match t.clone() {
                    // Add value to declaration type
                    Token::IdentOrKeyword(value) => match declare_type.as_ref() {
                        "Variable" => stage.variables.push(value),
                        "List" => stage.lists.push(value),
                        "Event" => stage.events.push(value),
                        _ => {
                            return Err(ParseError::InvalidDeclarationType(
                                declare_type.to_string(),
                            ))
                        }
                    },

                    // Ignore commas
                    Token::Comma => {}

                    // End of declaration
                    Token::BracketClose => break,

                    // Invalid token
                    _ => return Err(ParseError::InvalidTokenAfter(t, last)),
                }

                last = t;
            }
        },
    );

    Ok(())
}
