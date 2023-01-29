use nom::{bytes::complete::tag, IResult};
use luna_ast::keyword::Keyword;

use crate::parse::whitespace;

macro_rules! build_keyword_tags {
	($($name:tt, $tok:literal, $key:tt)*) => {
		$(
			#[doc = "Comsumes the leading and trailing whitespace around the `"]
			#[doc = $tok]
			#[doc = "` keyword."]
			#[inline]
			pub fn $name(input: &str) -> IResult<&str, Keyword> {
				let (input, _) = whitespace(tag($tok))(input)?;
				Ok((input, Keyword::$key))
			}
		)*
	};
}

build_keyword_tags! {
	kand, "and", And
	kbreak, "break", Break
	kdo, "do", Do
	kelse, "else", Else
	kelseif, "elseif", ElseIf
	kend, "end", End
	kfalse, "false", False
	kfor, "for", For
	kfunction, "function", Function
	kgoto, "goto", Goto
	kif, "if", If
	kin, "in", In
	klocal, "local", Local
	knil, "nil", Nil
	knot, "not", Not
	kor, "or", Or
	krepeat, "repeat", Repeat
	kreturn, "return", Return
	kthen, "then", Then
	ktrue, "true", True
	kuntil, "until", Until
	kwhile, "while", While
}
