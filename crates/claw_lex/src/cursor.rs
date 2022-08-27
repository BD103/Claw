use std::{
    iter::{Enumerate, Peekable},
    ops::{Deref, DerefMut},
    str::Chars,
};

pub struct Cursor<'a> {
    inner: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Cursor<'a> {
    pub fn new(script: &'a str) -> Self {
        Cursor {
            inner: script.chars().enumerate().peekable(),
        }
    }
}

impl<'a> Deref for Cursor<'a> {
    type Target = Peekable<Enumerate<Chars<'a>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Cursor<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
