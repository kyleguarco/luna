use core::{iter::Peekable, str::Chars};

use crate::{
    error::LexError,
    token::{Token, TokenType},
};

/// The iterating type for our lexer.
/// The lexer will stop iterating should it return this types' `Err` type.
pub type LexResult<'src> = Result<Token<'src>, LexError<'src>>;

pub struct Lexer<'src> {
    source: &'src str,
    chars: Peekable<Chars<'src>>,
    line: usize,
    col: usize,
    curr: usize,
    /// The beginning index of the current lexeme
    last: usize,
}

impl<'src> Lexer<'src> {
    pub fn new<T: AsRef<str>>(source: &'src T) -> Self {
        let source = source.as_ref();
        Self {
            source: source.as_ref(),
            chars: source.chars().peekable(),
            line: 1,
            col: 0,
            curr: 0,
            last: 0,
        }
    }

    fn new_lexeme(&mut self) {
        self.last = self.curr;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let Some(c) = self.chars.peek() else {
                break;
            };
            match c {
                ' ' | '\t' | '\x0C' | '\x0B' => {}
                '\n' => {
                    self.line += 1;
                    self.col = 0;
                }
                _ => break,
            }
            self.advance();
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.col += 1;
        self.curr += 1;
        self.chars.next()
    }

    fn advance_if(&mut self, expected: impl Fn(&char) -> bool) {
        while self.chars.next_if(&expected).is_some() {
            self.curr += 1;
            self.col += 1;
        }
    }

    fn accept(&mut self, ttype: TokenType) -> Option<LexResult<'src>> {
        let lexeme = self.lexeme();
        let tok = Some(Ok(Token::new(ttype, lexeme, self.line, self.col)));
        tok
    }

    #[inline]
    fn reject(&self) -> Option<LexResult<'src>> {
        Some(Err(LexError::new(self.lexeme(), self.line, self.col)))
    }

    #[inline]
    fn lexeme(&self) -> &'src str {
        // SAFETY: self.curr is guaranteed to stop at the last character.
        unsafe { self.source.get_unchecked(self.last..self.curr) }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = LexResult<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.new_lexeme();
        let curr = self.advance()?;

        match curr {
            '+' => self.accept(TokenType::Plus),
            '-' => self.accept(TokenType::Minus),
            '*' => self.accept(TokenType::Star),
            '/' => self.accept(TokenType::Slash),
            '%' => self.accept(TokenType::Percent),
            '^' => self.accept(TokenType::Caret),
            '#' => self.accept(TokenType::Pound),
            '&' => self.accept(TokenType::Amphersand),
            '~' => self.accept(TokenType::Tilde),
            '|' => self.accept(TokenType::Pipe),
            '<' => self.accept(TokenType::LessThan),
            '>' => self.accept(TokenType::GreaterThan),
            '=' => self.accept(TokenType::Equal),
            '(' => self.accept(TokenType::LeftParen),
            ')' => self.accept(TokenType::RightParen),
            '{' => self.accept(TokenType::LeftBrace),
            '}' => self.accept(TokenType::RightBrace),
            '[' => self.accept(TokenType::LeftBracket),
            ']' => self.accept(TokenType::RightBracket),
            '.' => self.accept(TokenType::Dot),
            ':' => self.accept(TokenType::Colon),
            ',' => self.accept(TokenType::Comma),
            ';' => self.accept(TokenType::Semicolon),
            _ if curr.is_alphabetic() => {
                self.advance_if(|c| c.is_alphanumeric());
                self.accept(TokenType::Identifier)
            }
            _ => self.reject(),
        }
    }
}
