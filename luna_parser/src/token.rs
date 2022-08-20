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
    pub(crate) fn new(ttype: TokenType, lexeme: &'src str, line: usize, col: usize) -> Self {
        Self {
            ttype,
            lexeme,
            line,
            col,
        }
    }

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
