use crate::combinator::ws0;
use nom::{
	bytes::complete::tag, error::ParseError, AsChar, Compare, IResult, InputLength, InputTake,
	InputTakeAtPosition,
};

/// A collection of the possible keywords in Lua.
pub enum Keyword {
	And,
	Break,
	Do,
	Else,
	ElseIf,
	End,
	False,
	For,
	Function,
	Goto,
	If,
	In,
	Local,
	Nil,
	Not,
	Or,
	Repeat,
	Return,
	Then,
	True,
	Until,
	While,
}

impl Keyword {
	/// Convert a keyword type to its corresponding string literal.
	pub fn literal(&self) -> &'static str {
		match self {
			Keyword::And => "and",
			Keyword::Break => "break",
			Keyword::Do => "do",
			Keyword::Else => "else",
			Keyword::ElseIf => "elseif",
			Keyword::End => "end",
			Keyword::False => "false",
			Keyword::For => "for",
			Keyword::Function => "function",
			Keyword::Goto => "goto",
			Keyword::If => "if",
			Keyword::In => "in",
			Keyword::Local => "local",
			Keyword::Nil => "nil",
			Keyword::Not => "not",
			Keyword::Or => "or",
			Keyword::Repeat => "repeat",
			Keyword::Return => "return",
			Keyword::Then => "then",
			Keyword::True => "true",
			Keyword::Until => "until",
			Keyword::While => "while",
		}
	}
}

/// An implementation of `Into` for convienence.
impl Into<&'static str> for Keyword {
	fn into(self) -> &'static str {
		self.literal()
	}
}

/// Strips the whitespace surrounding a keyword and returns the parsed input,
/// along with the string literal corresponding to the keyword.
pub fn keyword<Input, Error: ParseError<Input>>(
	key: Keyword,
) -> impl FnMut(Input) -> IResult<Input, Input, Error>
where
	Input: Clone + InputTake + InputTakeAtPosition + InputLength + Compare<&'static str>,
	<Input as InputTakeAtPosition>::Item: Clone + AsChar,
{
	ws0(tag(key.literal()))
}

// const KAND: &str = "and";
// const KBREAK: &str = "break";
// const KDO: &str = "do";
// const KELSE: &str = "else";
// const KELSEIF: &str = "elseif";
// const KEND: &str = "end";
// const KFALSE: &str = "false";
// const KFOR: &str = "for";
// const KFUNCTION: &str = "function";
// const KGOTO: &str = "goto";
// const KIF: &str = "if";
// const KIN: &str = "in";
// const KLOCAL: &str = "local";
// const KNIL: &str = "nil";
// const KNOT: &str = "not";
// const KOR: &str = "or";
// const KREPEAT: &str = "repeat";
// const KRETURN: &str = "return";
// const KTHEN: &str = "then";
// const KTRUE: &str = "true";
// const KUNTIL: &str = "until";
// const KWHILE: &str = "while";
