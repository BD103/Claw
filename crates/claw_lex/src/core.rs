use crate::{Cursor, ErrorType, LexError, LexResult, Span, Token, TokenType};

pub fn tokenize(script: String) -> LexResult<Vec<Token>> {
    if script.is_empty() {
        return Err(crate::error::empty_script_error());
    }

    let mut it = Cursor::new(&script);
    let mut response = Vec::new();

    for (start, c) in it.by_ref() {
        let mut end = start + 1;

        let token_type = match c {
            _ => TokenType::Unknown,
        };

        let span = Span::new(start, end);

        if let TokenType::Unknown = token_type {
            return Err(LexError::new(ErrorType::UnknownToken(c), &span, &script));
        }

        response.push(Token::new(token_type, span));
    }

    Ok(response)
}
