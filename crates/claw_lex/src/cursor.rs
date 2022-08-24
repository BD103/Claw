use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
};

/// A cursor that iterates over the characters of string.
///
/// Largely inspired from rustc_lexer, this keeps track of which line it is on.
/// This is mainly used for debugging code. Cursor also wraps the iterator in
/// [`Peekable`].
pub struct Cursor<I>
where
    I: Iterator<Item = char>,
{
    it: Peekable<I>,
    line: usize,
}

impl<I> Cursor<I>
where
    I: Iterator<Item = char>,
{
    /// Returns the current line the iterator is on.
    ///
    /// This is incremented every newline (`\n`) that is comsumed.
    pub fn line(&self) -> usize {
        self.line
    }
}

impl<I> Iterator for Cursor<I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.it.next();

        // Count line
        if let Some('\n') = res {
            self.line += 1;
        }

        res
    }
}

impl<I> Deref for Cursor<I>
where
    I: Iterator<Item = char>,
{
    type Target = Peekable<I>;

    fn deref(&self) -> &Self::Target {
        &self.it
    }
}

impl<I> DerefMut for Cursor<I>
where
    I: Iterator<Item = char>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.it
    }
}

impl<I> From<Peekable<I>> for Cursor<I>
where
    I: Iterator<Item = char>,
{
    fn from(it: Peekable<I>) -> Self {
        // Starts on line 1
        Cursor { it, line: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Chars;

    const TEXT: &str = "1\n22\n333\n";

    fn make_cursor() -> Cursor<Chars<'static>> {
        Cursor::from(TEXT.chars().peekable())
    }

    #[test]
    fn new() {
        let cursor = make_cursor();
        assert_eq!(cursor.collect::<String>(), TEXT.to_string());
    }

    #[test]
    fn lines() {
        let mut cursor = make_cursor();

        assert_eq!(cursor.line(), 1);

        // Needs to consume '\n' before line updates
        cursor.next();
        assert_eq!(cursor.line(), 1);
        cursor.next();

        assert_eq!(cursor.line(), 2);

        cursor.next();
        cursor.next();
        cursor.next();

        assert_eq!(cursor.line(), 3);

        cursor.next();
        cursor.next();
        cursor.next();
        cursor.next();

        assert_eq!(cursor.line(), 4);
    }

    #[test]
    fn iterating() {
        let cursor = make_cursor();

        assert_eq!(cursor.count(), TEXT.len());
    }

    #[test]
    fn dereferencing() {
        // Syntactical sugar
        let mut cursor = make_cursor();
        assert_eq!((*cursor).nth(5).unwrap(), '3');

        // Explicit
        let mut cursor = make_cursor();
        assert_eq!(cursor.deref_mut().nth(5).unwrap(), '3');
    }
}
