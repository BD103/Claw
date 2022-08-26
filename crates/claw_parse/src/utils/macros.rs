/// Wrapper for the error handling a token iterator.
///
/// This is commonly used to ensure the next token is the given type.
/// This macro uses the `return` keyword for error handling.
/// Make sure to use it in a function that returns
/// [`ParseResult`](crate::parse::ParseResult).
///
/// # Arguments
///
/// - `it`, The name of the token iterator.
/// - `token_next`, A pattern that the next token should match or else an error is returned.
/// - `token_from`, An expression representing the previous token. This is used in error handling.
/// - `func`, A block of code that is run if no errors occur.
macro_rules! ensure_next_token {
    ($it:ident, $token_next:pat, $token_from:expr, $func:expr) => {
        // Ensure there is a token next
        if let ::std::option::Option::Some(__t) = $it.next() {
            // Ensure next token is given type
            if let $token_next = __t {
                // Run function
                $func
            } else {
                return ::std::result::Result::Err($crate::ParseError::InvalidTokenAfter(
                    __t,
                    $token_from,
                ));
            }
        } else {
            return ::std::result::Result::Err($crate::ParseError::NoTokenAfter($token_from));
        }
    };
    // Allows for comma at the end
    ($it:ident, $token_next:pat, $token_from:expr, $func:expr,) => {
        ensure_next_token!($it, $token_next, $token_from, $func);
    };
}

pub(crate) use ensure_next_token;
