use core::{ops::Deref, str::Chars, iter::Peekable};

#[rustfmt::skip]
#[derive(Clone, Debug)]
enum TokenType {
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
	LessEqual, GreaterEqual, DoubleColon, DoubleDots, TripleDots
}

#[derive(Clone, Debug)]
pub struct Token {
    ttype: TokenType,
    line: usize,
    col: usize,
}

pub struct Lexer<'src> {
    source: Peekable<Chars<'src>>,
    position: usize,
    line: usize,
    col: usize,
}

impl<'src, I> From<I> for Lexer<'src>
where
    Peekable<Chars<'src>>: From<I>,
{
    fn from(source: I) -> Self {
        Lexer {
            source: source.into(),
            position: 0,
            line: 1,
            col: 1,
        }
    }
}

impl Lexer<'_> {
	fn new_token(&self, ttype: TokenType) -> Token {
		Token { ttype, line: self.line, col: self.col }
	}
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == 10 {
			return None
		}

		self.position += 1;
		Some(self.new_token(TokenType::And))
    }
}
