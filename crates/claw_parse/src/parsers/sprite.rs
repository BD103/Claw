use crate::{parse_single, utils::ensure_next_token, Context, ParseResult, Scope, Stage};
use claw_lex::Token;
use std::iter::Peekable;

pub fn parse_sprite<I: Iterator<Item = Token>>(
    it: &mut Peekable<I>,
    stage: &mut Stage,
    ctx: &mut Context,
) -> ParseResult<()> {
    ensure_next_token!(
        it,
        Token::IdentOrKeyword(value),
        Token::IdentOrKeyword("sprite".to_string()),
        {
            ensure_next_token!(it, Token::BracketOpen, Token::IdentOrKeyword(value), {
                ctx.scope = Scope::Sprite(value);

                while let Some(token) = it.next() {
                    match token {
                        Token::BracketClose => break,
                        _ => match parse_single(it, stage, ctx) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        },
                    }
                }
            });
        }
    );

    Ok(())
}
