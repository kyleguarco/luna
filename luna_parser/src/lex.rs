use core::{iter::Peekable, ops::Range, str::Chars};

use luna_types::num::{LuaInteger, LuaNumber};

use crate::{
    cursor::Cursor,
    error::{LexError, LexErrorType},
    token::{Token, TokenType},
};

/// The iterating type for our lexer.
/// The lexer will stop iterating should it return this types' `Err` type.
pub type LexResult<'src> = Result<Token<'src>, LexError<'src>>;

#[derive(Default, Clone, Copy)]
struct LexemeSlice {
    start: usize,
    end: usize,
}

impl LexemeSlice {
    fn advance(&mut self) {
        self.end += 1;
    }

    fn reset(&mut self) {
        self.start = self.end;
    }

    fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    chars: Peekable<Chars<'src>>,
    cursor: Cursor,
    slice: LexemeSlice,
}

impl<'src> Lexer<'src> {
    pub fn new<T: AsRef<str>>(source: &'src T) -> Self {
        let source = source.as_ref();
        Self {
            source: source.as_ref(),
            chars: source.chars().peekable(),
            cursor: Cursor::default(),
            slice: LexemeSlice::default(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.cursor.advance();
        self.slice.advance();
        self.chars.next()
    }

    fn advance_if(&mut self, expected: impl Fn(&char) -> bool) -> Option<&char> {
        while self.chars.next_if(&expected).is_some() {
            self.slice.advance();
            self.cursor.advance();
        }

        self.chars.peek()
    }

    fn skip_whitespace(&mut self) {
        loop {
            let Some(c) = self.chars.peek() else {
                break;
            };
            match c {
                ' ' | '\t' | '\x0C' | '\x0B' => {}
                '\n' => {
                    self.cursor.newline();
                }
                _ => break,
            }
            self.advance();
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    #[inline]
    fn tokenize(&self, ttype: TokenType) -> Option<LexResult<'src>> {
        Some(Ok(Token::new(ttype, self.lexeme(), self.cursor)))
    }

    #[inline]
    fn error(&self, reason: LexErrorType) -> Option<LexResult<'src>> {
        Some(Err(LexError::new(reason, self.lexeme(), self.cursor)))
    }

    #[inline]
    fn lexeme(&self) -> &'src str {
        self.source.get(self.slice.as_range()).unwrap()
    }

    fn lex_keyword_identifier(&mut self) -> Option<LexResult<'src>> {
        self.advance_if(|c| c.is_alphanumeric())?;
        let ttype = TokenType::try_keyword(self.lexeme()).unwrap_or(TokenType::Identifier);
        self.tokenize(ttype)
    }

    fn lex_string_literal(&mut self) -> Option<LexResult<'src>> {
        self.advance_if(|c| c != &'\"' || c == &'\n')?;
        // TODO: Error on strings that aren't closed
        // if self.advance() != Some('\"') {
        //     return self.reject();
        // }
        self.advance()?;
        self.tokenize(TokenType::String)
    }

    fn lex_numeric_literal(&mut self) -> Option<LexResult<'src>> {
        self.advance_if(|c| c.is_digit(10) || c == &'E' || c == &'e' || c == &'.')?;

        let lex = self.lexeme();

        match lex.parse::<LuaInteger>() {
            Ok(num) => self.tokenize(TokenType::Integer(num)),
            Err(_) => match lex.parse::<LuaNumber>() {
                Ok(int) => self.tokenize(TokenType::Float(int)),
                Err(_) => self.error(LexErrorType::InvalidLiteral),
            },
        }
    }

    fn lex_line_comment(&mut self) {
        self.advance_if(|c| c != &'\n');
        self.advance();
        self.cursor.newline();
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = LexResult<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.slice.reset();
        let curr = self.advance()?;

        match curr {
            '+' => self.tokenize(TokenType::Plus),
            '-' => match self.peek() {
                Some(&'-') => {
                    self.lex_line_comment();
                    self.next()
                }
                _ => self.tokenize(TokenType::Minus),
            },
            '*' => self.tokenize(TokenType::Star),
            '/' => self.tokenize(TokenType::Slash),
            '%' => self.tokenize(TokenType::Percent),
            '^' => self.tokenize(TokenType::Caret),
            '#' => self.tokenize(TokenType::Pound),
            '&' => self.tokenize(TokenType::Amphersand),
            '~' => self.tokenize(TokenType::Tilde),
            '|' => self.tokenize(TokenType::Pipe),
            '<' => self.tokenize(TokenType::LessThan),
            '>' => self.tokenize(TokenType::GreaterThan),
            '=' => self.tokenize(TokenType::Equal),
            '(' => self.tokenize(TokenType::LeftParen),
            ')' => self.tokenize(TokenType::RightParen),
            '{' => self.tokenize(TokenType::LeftBrace),
            '}' => self.tokenize(TokenType::RightBrace),
            '[' => self.tokenize(TokenType::LeftBracket),
            ']' => self.tokenize(TokenType::RightBracket),
            '.' => self.tokenize(TokenType::Dot),
            ':' => self.tokenize(TokenType::Colon),
            ',' => self.tokenize(TokenType::Comma),
            ';' => self.tokenize(TokenType::Semicolon),
            '\"' => self.lex_string_literal(),
            _ if curr.is_digit(10) => self.lex_numeric_literal(),
            _ if curr.is_alphabetic() => self.lex_keyword_identifier(),
            _ => self.error(LexErrorType::UnknownSequence),
        }
    }
}
