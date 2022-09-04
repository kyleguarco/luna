use core::ops::Deref;

use crate::token::{Token, TokenType};

pub struct Lexer<'src> {
    source: &'src str,
    /// Position of the last successful parse
    last: usize,
    /// The cursor (position in the source string)
    position: usize,
    line: usize,
    col: usize,
}

impl<'src> Lexer<'src> {
    pub fn new<T: Deref<Target = str>>(source: &'src T) -> Self {
        Self {
            source: source.as_ref(),
            last: 0,
            position: 0,
            line: 1,
            col: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self, peek: usize) -> &'src str {
        let slice = if self.position + peek < self.source.len() {
            self.position..self.position + peek
        } else {
            self.position..self.source.len()
        };
        // Unwrap is safe here, since we check the length of the lookahead
        // before returning it. We return a slice to the end of the source
        // if the length is invalid.
        self.source.get(slice).unwrap()
    }

    fn tokenize(&mut self, ttype: TokenType) -> Token<'src> {
        let token = Token::new(
            ttype,
            self.source.get(self.last..self.position).unwrap(),
            self.line,
            self.col,
        );
        self.last = self.position;
        token
    }

    fn new_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn skip_line(&mut self) {
        while self.peek(1) != "\n" {
            self.position += 1;
        }
        self.last = self.position;
        self.new_line();
    }

    fn advance(&mut self, seek: usize) -> &'src str {
        let slice = self.peek(seek);
        self.position += seek;
        self.col += seek;
        slice
    }

    fn parse_string(&mut self) -> Option<Token<'src>> {
        let quote = self.source.get(self.last..self.position).unwrap();

        loop {
            let cur = self.peek(1);

            // TODO: This allows newlines in all strings, but that shouldn't be possible
            if cur == "\n" {
                self.new_line();
            }

            if cur == quote || self.is_at_end() {
                break;
            }

            self.advance(1);
        }

        if self.is_at_end() {
            None
        } else {
            let tok = Some(self.tokenize(TokenType::String));
            self.advance(1);
            tok
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        match self.advance(1) {
            "+" => Some(self.tokenize(TokenType::Plus)),
            "*" => Some(self.tokenize(TokenType::Star)),
            "%" => Some(self.tokenize(TokenType::Percent)),
            "^" => Some(self.tokenize(TokenType::Caret)),
            "#" => Some(self.tokenize(TokenType::Pound)),
            "&" => Some(self.tokenize(TokenType::Amphersand)),
            "|" => Some(self.tokenize(TokenType::Pipe)),
            "(" => Some(self.tokenize(TokenType::LeftParen)),
            ")" => Some(self.tokenize(TokenType::RightParen)),
            "{" => Some(self.tokenize(TokenType::LeftBrace)),
            "}" => Some(self.tokenize(TokenType::RightBrace)),
            "[" => Some(self.tokenize(TokenType::LeftBracket)),
            "]" => Some(self.tokenize(TokenType::RightBracket)),
            "," => Some(self.tokenize(TokenType::Comma)),
            ";" => Some(self.tokenize(TokenType::Semicolon)),
            "-" => match self.peek(1) {
                "-" => {
                    self.skip_line();
                    self.next()
                }
                _ => Some(self.tokenize(TokenType::Minus)),
            },
            "/" => match self.peek(1) {
                "/" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::DoubleSlash))
                }
                _ => Some(self.tokenize(TokenType::Slash)),
            },
            "~" => match self.peek(1) {
                "=" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::NotEqual))
                }
                _ => Some(self.tokenize(TokenType::Tilde)),
            },
            "<" => match self.peek(1) {
                "=" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::LessEqual))
                }
                _ => Some(self.tokenize(TokenType::LessThan)),
            },
            ">" => match self.peek(1) {
                "=" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::GreaterEqual))
                }
                _ => Some(self.tokenize(TokenType::GreaterThan)),
            },
            "=" => match self.peek(1) {
                "=" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::Equality))
                }
                _ => Some(self.tokenize(TokenType::Equal)),
            },
            "." => match self.peek(1) {
                "." => match self.peek(2) {
                    ".." => {
                        self.advance(2);
                        Some(self.tokenize(TokenType::TripleDots))
                    }
                    _ => {
                        self.advance(1);
                        Some(self.tokenize(TokenType::DoubleDots))
                    }
                },
                _ => Some(self.tokenize(TokenType::Dot)),
            },
            ":" => match self.peek(1) {
                ":" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::DoubleColon))
                }
                _ => Some(self.tokenize(TokenType::Colon)),
            },
            "\"" | "\'" => self.parse_string(),
            // Ignore the whitespace and return the result of the next iteration
            " " | "\x0C" | "\t" | "\x0B" => self.next(),
            "\n" | "\r" => {
                self.new_line();
                self.next()
            }
            _ => None,
        }
    }
}
