use nom::{bytes::complete::tag, IResult};

use crate::parse::whitespace;

macro_rules! build_keyword_tags {
	($($name:tt, $tok:literal)*) => {
		$(
			#[doc = "Comsumes the leading and trailing whitespace around the `"]
			#[doc = $tok]
			#[doc = "` keyword."]
			#[inline]
			pub fn $name(i: &str) -> IResult<&str, &str> {
				whitespace(tag($tok))(i)
			}
		)*
	};
}

build_keyword_tags! {
	kand, "and"
	kbreak, "break"
	kdo, "do"
	kelse, "else"
	kelseif, "elseif"
	kend, "end"
	kfalse, "false"
	kfor, "for"
	kfunction, "function"
	kgoto, "goto"
	kif, "if"
	kin, "in"
	klocal, "local"
	knil, "nil"
	knot, "not"
	kor, "or"
	krepeat, "repeat"
	kreturn, "return"
	kthen, "then"
	ktrue, "true"
	kuntil, "until"
	kwhile, "while"
}
