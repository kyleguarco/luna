use core::ops::Deref;

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum TokenType {
	// Keywords
	And, Break, Do, Else, ElseIf, End, False, For, Function,
	Goto, If, In, Local, Nil, Not, Or, Repeat, Return, Then,
	True, Until, While,
	// Terminal Symbols (single)
	Plus, Minus, Star, Slash, Percent, Caret, Pound, Amphersand,
	Tilde, Pipe, LessThan, GreaterThan, Equal, LeftParen,
	RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
	Dot, Colon, Comma, Semicolon,
	// Terminal Symbols (greater than one)
	ShiftLeft, ShiftRight, DoubleSlash, Equality, NotEqual,
	LessEqual, GreaterEqual, DoubleColon, DoubleDots, TripleDots,
    // Literals
    Identifier, String, Number, Float
}

#[derive(Clone, Debug)]
pub struct Token<'src> {
    ttype: TokenType,
    lexeme: &'src str,
    line: usize,
    col: usize,
}

impl<'src> Token<'src> {
    pub fn token_type(&self) -> &TokenType {
        &self.ttype
    }

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn lexeme(&self) -> &'src str {
        self.lexeme
    }
}

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

    fn is_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self, peek: usize) -> &'src str {
        let slice = if self.position + peek >= self.source.len() {
            self.position..self.source.len() - 1
        } else {
            self.position..self.position + peek
        };
        self.source.get(slice).unwrap()
    }

    fn tokenize(&mut self, ttype: TokenType) -> Token<'src> {
        let token = Token {
            ttype,
            lexeme: self.source.get(self.last..self.position).unwrap(),
            line: self.line,
            col: self.col,
        };
        self.last = self.position;
        token
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
        if self.is_end() {
            return None;
        }

        match self.advance(1) {
            // Terminal Symbols
            "+" => Some(self.tokenize(TokenType::Plus)),
            "-" => Some(self.tokenize(TokenType::Minus)),
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
            // Ignore the whitespace and return the result of tne next iteration
            " " | "\x0C" | "\t" | "\x0B" => self.next(),
            "\n" | "\r" => {
                self.line += 1;
                self.col = 0;
                self.next()
            },
            _ => None,
        }
    }
}
