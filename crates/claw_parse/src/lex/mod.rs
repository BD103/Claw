pub fn lex<'a>(source: &'a str) -> Tokenizer<'a> {
    Tokenizer::new(source)
}

pub struct Tokenizer<'a> {
    cursor: usize,
    source: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Tokenizer {
            cursor: 0,
            source,
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[non_exhaustive]
pub enum Token {
    EOF,
}
