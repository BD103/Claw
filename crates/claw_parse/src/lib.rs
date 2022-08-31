pub mod ast;

/// Represents a project defined by a script.
pub struct Project<'a> {
    pub stage: ast::Target<'a>,
    pub sprites: Vec<ast::Target<'a>>,
}

/// Parses an iterator of [`Token`](claw_lex::Token)s into a [`Project`].
pub fn parse<'a>(_tokens: impl Iterator<Item = claw_lex::Token> + 'a) -> Project<'a> {
    todo!()
}
