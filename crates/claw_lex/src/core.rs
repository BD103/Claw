use crate::{is_whitespace, Cursor, ErrorType, LexError, LexResult, Span, Token, TokenType};

pub fn tokenize(script: String) -> LexResult<Vec<Token>> {
    if script.is_empty() {
        return Err(crate::error::empty_script_error());
    }

    let mut it = Cursor::new(&script);
    let mut response = Vec::new();

    while let Some((start, c)) = it.next() {
        let mut end = start + 1;

        let token_type = match c {
            '(' => TokenType::ParanOpen,
            ')' => TokenType::ParanClose,
            '{' => TokenType::BracketOpen,
            '}' => TokenType::BracketClose,

            '@' => TokenType::AtSign,
            '$' => TokenType::EnumSign,
            ',' => TokenType::Comma,
            ';' => TokenType::Semi,

            ':' => {
                end += 1;

                if let Some(&(_, ':')) = it.peek() {
                    it.next();

                    TokenType::DoubleColon
                } else {
                    return Err(LexError::new(ErrorType::SingleColon, &Span::new(start, end), &script));
                }
            }

            _ if c.is_ascii_alphabetic() => {
                while let Some(&(new_end, c)) = it.peek() {
                    end = new_end;

                    if c.is_ascii_alphanumeric() || c == '_' {
                        it.next();
                    } else {
                        break;
                    }
                }

                TokenType::IdentOrKeyword
            }

            _ if is_whitespace(c) => TokenType::Whitespace,
            _ => TokenType::Unknown,
        };

        let span = Span::new(start, end);

        if let TokenType::Unknown = token_type {
            return Err(LexError::new(ErrorType::UnknownToken(c), &span, &script));
        } else if let TokenType::Whitespace = token_type {
            // Skip whitespace
            continue;
        }

        response.push(Token::new(token_type, span));
    }

    Ok(response)
}
