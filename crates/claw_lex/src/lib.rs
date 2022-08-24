mod cursor;
mod errors;
mod token;

#[cfg(test)]
mod tests;

pub use self::{cursor::Cursor, errors::LexError, token::Token};

/// The type returned by [`tokenize`] and other functions in the
/// claw_lex crate.
pub type LexResult<T> = Result<T, LexError>;

/// Returns true if given character is whitespace, else false.
///
/// This is taken directly from rustc_lexer.
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

fn tokenize_string<I: Iterator<Item = char>>(it: &mut Cursor<I>) -> LexResult<Token> {
    fn is_next_newline<I: Iterator<Item = char>>(it: &mut Cursor<I>) -> bool {
        match it.peek() {
            Some(&'\n') => true,
            _ => false,
        }
    }

    let mut text = String::new();
    // Technically will always be last line, but this does the same thing.
    let string_line = it.line();

    while let Some(c) = it.next() {
        // Allow escaping / ignoring next character
        if c == '\\' {
            println!("Escaping");
            if let Some(c) = it.next() {
                text.push(c);

                // Windows newlines are weird
                if c == '\r' {
                    if let Some(&'\n') = it.peek() {
                        it.next();
                        text.push('\n');
                    }
                }
            }

            continue;
        }

        match c {
            // End of string
            '"' => return Ok(Token::Text(text)),

            // Newlines, with support for Windows weirdness
            '\n' => return Err(LexError::StringNewline(it.line())),
            '\r' if is_next_newline(it) => return Err(LexError::StringNewline(it.line())),

            // Add character to string
            c => text.push(c),
        }
    }

    // Will only be returned if closing '"' is not found
    Err(LexError::NoClosingQuotationMark(string_line))
}

fn tokenize_number<I: Iterator<Item = char>>(it: &mut Cursor<I>, c: char) -> LexResult<Token> {
    let mut number = String::from(c);
    let mut is_float = false;

    while let Some(&c) = it.peek() {
        // While next character is digit / decimal point
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            it.next();
        } else {
            break;
        }

        if c == '.' {
            is_float = true;
        }
    }

    if is_float {
        Ok(Token::Decimal(match number.parse() {
            Ok(n) => n,
            Err(e) => return Err(LexError::FloatParse(e, it.line())),
        }))
    } else {
        Ok(Token::Integer(match number.parse() {
            Ok(n) => n,
            Err(e) => return Err(LexError::IntegerParse(e, it.line())),
        }))
    }
}

/// Converts the given `script` into a list of [`Token`]s.
///
/// This is the main function of the claw_lex crate.
pub fn tokenize(script: String) -> LexResult<Vec<Token>> {
    let mut it = Cursor::from(script.chars().peekable());
    let mut response = Vec::new();

    while let Some(c) = it.next() {
        let token = match c {
            // Single character
            '(' => Token::ParanOpen,
            ')' => Token::ParanClose,
            '{' => Token::BracketOpen,
            '}' => Token::BracketClose,

            '+' => Token::Add,
            // Sub is handled with number
            '*' => Token::Mul,
            '/' => Token::Div,

            ':' => {
                if let Some(&':') = it.peek() {
                    it.next();

                    // Ensure not triple-colon or more
                    if let Some(&':') = it.peek() {
                        return Err(LexError::TripleColon(it.line()));
                    } else {
                        Token::DoubleColon
                    }
                } else {
                    Token::Colon
                }
            }
            ';' => Token::Semi,
            ',' => Token::Comma,
            '@' => Token::AtSign,
            '$' => Token::EnumSign,

            // Comparisons
            '=' => {
                if let Some(&'=') = it.peek() {
                    it.next();
                    Token::Eq
                } else {
                    return Err(LexError::SingleEq(it.line()));
                }
            }
            '!' => {
                // '!' can be used for both negating bools and unequal comparisons
                if let Some(&'=') = it.peek() {
                    it.next();
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            '&' => {
                if let Some(&'&') = it.peek() {
                    it.next();
                    Token::And
                } else {
                    return Err(LexError::SingleAnd(it.line()));
                }
            }
            '|' => {
                if let Some(&'|') = it.peek() {
                    it.next();
                    Token::Or
                } else {
                    return Err(LexError::SingleOr(it.line()));
                }
            }

            // String
            '"' => match tokenize_string(&mut it) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },

            // Unsigned number
            c if c.is_ascii_digit() => match tokenize_number(&mut it, c) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },

            // Signed number
            '-' => {
                if let Some(&c) = it.peek() {
                    // If minus is part of a number or not
                    if c.is_ascii_digit() {
                        match tokenize_number(&mut it, '-') {
                            Ok(t) => t,
                            Err(e) => return Err(e),
                        }
                    } else {
                        Token::Sub
                    }
                } else {
                    Token::Sub
                }
            }

            // Identifier / Keyword
            _ if c.is_ascii_alphabetic() => {
                // c is already consumed, so need to prefix it to text
                let mut text = String::from(c);

                while let Some(&c) = it.peek() {
                    // Can contain numbers and underscore, just can't start with them
                    if c.is_ascii_alphanumeric() || c == '_' {
                        text.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }

                Token::IdentOrKeyword(text)
            }

            // Other
            _ if is_whitespace(c) => Token::Whitespace,
            _ => Token::Unknown,
        };

        if let Token::Unknown = token {
            return Err(LexError::UnkownToken(c, it.line()));
        } else if let Token::Whitespace = token {
            // Skip whitespace
            continue;
        }

        response.push(token);
    }

    Ok(response)
}
