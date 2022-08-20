#![no_std]

pub mod token;

use core::ops::Deref;

use token::{Token, TokenType};

pub struct Lexer<'src> {
    source: &'src str,
    last: usize,
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
        self.last = self.position;
        self.line += 1;
        self.col = 0;
    }

    fn skip_line(&mut self) {
        while self.peek(1) != "\n" {
            self.position += 1;
        }
    }

    fn advance(&mut self, seek: usize) -> &'src str {
        let slice = self.peek(seek);
        self.position += seek;
        self.col += seek;
        slice
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
            "-" => match self.peek(1) {
                "-" => {
                    self.skip_line();
                    self.new_line();
                    self.next()
                }
                _ => Some(self.tokenize(TokenType::Minus)),
            },
            "*" => Some(self.tokenize(TokenType::Star)),
            "/" => match self.peek(1) {
                "/" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::DoubleSlash))
                }
                _ => Some(self.tokenize(TokenType::Slash)),
            },
            "%" => Some(self.tokenize(TokenType::Percent)),
            "^" => Some(self.tokenize(TokenType::Caret)),
            "#" => Some(self.tokenize(TokenType::Pound)),
            "&" => Some(self.tokenize(TokenType::Amphersand)),
            "~" => match self.peek(1) {
                "=" => {
                    self.advance(1);
                    Some(self.tokenize(TokenType::NotEqual))
                }
                _ => Some(self.tokenize(TokenType::Tilde)),
            },
            "|" => Some(self.tokenize(TokenType::Pipe)),
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
            "(" => Some(self.tokenize(TokenType::LeftParen)),
            ")" => Some(self.tokenize(TokenType::RightParen)),
            "{" => Some(self.tokenize(TokenType::LeftBrace)),
            "}" => Some(self.tokenize(TokenType::RightBrace)),
            "[" => Some(self.tokenize(TokenType::LeftBracket)),
            "]" => Some(self.tokenize(TokenType::RightBracket)),
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
            "," => Some(self.tokenize(TokenType::Comma)),
            ";" => Some(self.tokenize(TokenType::Semicolon)),
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
