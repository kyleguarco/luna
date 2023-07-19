use luna_ast::terminal::{LiteralString, Name, Numeral};
use nom::{
	bytes::complete::is_not,
	character::complete::{alpha1, char as wschar, digit1},
	combinator::{map_res, verify},
	sequence::delimited,
	Parser,
};

use crate::{
	combinator::{list, negate},
	terminal::keyword::is_keyword,
	IRes, In,
};

use self::string::COMMA;

pub mod keyword;
pub mod string;

pub(crate) fn name(input: In) -> IRes<Name> {
	dbg!(input);
	// TODO: Support ASCII symbols and numbers.
	verify(alpha1, negate(is_keyword))
		.map(String::from)
		.map(Name)
		.parse(input)
}

pub(crate) fn numeral(input: In) -> IRes<Numeral> {
	dbg!(input);
	// TODO: Support floats! And Hex!
	map_res(digit1, str::parse)
		.map(Numeral::Integer)
		.parse(input)
}

pub(crate) fn literal_string(input: In) -> IRes<LiteralString> {
	dbg!(input);
	// TODO: Support escpaed strings and multiline strings.
	delimited(wschar('"'), parse_literal, wschar('"'))
		.map(String::from)
		.map(LiteralString)
		.parse(input)
}

pub fn namelist(input: In) -> IRes<Vec<Name>> {
	dbg!(input);
	list(wschar(COMMA), name)(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
/// From https://github.com/rust-bakery/nom/blob/main/examples/string.rs
fn parse_literal(input: In) -> IRes<In> {
	dbg!(input);
	// `is_not` parses a string of 0 or more characters that aren't one of the
	// given characters.
	let not_quote_slash = is_not("\"\\");

	// `verify` runs a parser, then runs a verification function on the output of
	// the parser. The verification function accepts out output only if it
	// returns true. In this case, we want to ensure that the output of is_not
	// is non-empty.
	verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}
