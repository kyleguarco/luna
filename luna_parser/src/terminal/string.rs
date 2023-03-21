pub enum SingleSymbol {
	Plus,
	Minus,
	Star,
	Slash,
	Percent,
	Caret,
	Octothorpe,
	Amphersand,
	Tilde,
	Pipe,
	Less,
	Greater,
	Equals,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,
	Semicolon,
	Colon,
	Comma,
	Dot,
}

impl SingleSymbol {
	pub const fn literal(&self) -> char {
		match self {
			Self::Plus => '+',
			Self::Minus => '-',
			Self::Star => '*',
			Self::Slash => '/',
			Self::Percent => '%',
			Self::Caret => '^',
			Self::Octothorpe => '#',
			Self::Amphersand => '&',
			Self::Tilde => '~',
			Self::Pipe => '|',
			Self::Less => '<',
			Self::Greater => '>',
			Self::Equals => '=',
			Self::LeftParen => '(',
			Self::RightParen => ')',
			Self::LeftBrace => '{',
			Self::RightBrace => '}',
			Self::LeftBracket => '[',
			Self::RightBracket => ']',
			Self::Semicolon => ';',
			Self::Colon => ':',
			Self::Comma => ',',
			Self::Dot => '.',
		}
	}
}

pub enum LongSymbol {
	ShiftLeft,
	ShiftRight,
	DoubleSlash,
	IsEqual,
	NotEqual,
	LessEqual,
	GreaterEqual,
	DoubleColon,
	DoubleDot,
	TripleDot,
}

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const STAR: &str = "*";
pub const SLASH: &str = "/";
pub const PERCENT: &str = "%";
pub const CARET: &str = "^";
pub const OCTOTHORPE: &str = "#";
pub const AMPHERSAND: &str = "&";
pub const TILDE: &str = "~";
pub const PIPE: &str = "|";
pub const SHIFTLEFT: &str = "<<";
pub const SHIFTRIGHT: &str = ">>";
pub const DOUBLESLASH: &str = "//";
pub const ISEQUAL: &str = "==";
pub const NOTEQUAL: &str = "~=";
pub const LESSEQUAL: &str = "<=";
pub const GREATEREQUAL: &str = ">=";
pub const LESS: &str = "<";
pub const GREATER: &str = ">";
pub const EQUALS: &str = "=";
pub const LEFTPAREN: &str = "(";
pub const RIGHTPAREN: &str = ")";
pub const LEFTBRACE: &str = "{";
pub const RIGHTBRACE: &str = "}";
pub const LEFTBRACKET: &str = "[";
pub const RIGHTBRACKET: &str = "]";
pub const DOUBLECOLON: &str = "::";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";
pub const COMMA: &str = ",";
pub const DOT: &str = ".";
pub const DOUBLEDOT: &str = "..";
pub const TRIPLEDOT: &str = "...";
