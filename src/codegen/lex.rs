use core::ops::Deref;

use alloc::vec::Vec;

use super::reserved::Reserved;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    ttype: Reserved,
    line: usize,
    col: usize,
}

impl Token {
    fn new(ttype: Reserved, line: usize, col: usize) -> Self {
        Token { ttype, line, col }
    }
}

pub struct LexerState<'a> {
    source: &'a str,
}

impl<'a> LexerState<'a> {
    pub fn new<T: Deref<Target = str>>(source: &'a T) -> Self {
        LexerState { source }
    }

    pub fn to_tokens(self) -> Vec<Token> {
        let mut tokens = Vec::new();

        tokens
    }
}
