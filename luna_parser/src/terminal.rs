use luna_ast::types::{Identifier, Numeral, LiteralString};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, alphanumeric1},
	combinator::recognize,
	multi::many0_count,
	sequence::pair,
	IResult,
};

pub(crate) mod keyword;
pub(crate) mod string;

// https://github.com/rust-bakery/nom/blob/main/examples/string.rs

/// Parses identifiers (which share naming conventions with Rust).
/// Lua refers to identifiers as "Names" in their documentation.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#rust-style-identifiers
pub fn identifier(input: &str) -> IResult<&str, Identifier> {
	let (input, result) = recognize(pair(
		alt((alpha1, tag("_"))),
		many0_count(alt((alphanumeric1, tag("_")))),
	))(input)?;

	Ok((input, Identifier(result)))
}

/// Parses integers and floats.
///
/// Examples of valid integers: `3` `345` `0xff` `0xBEBADA`
/// <br>
/// Examples of valid floats: `3.0` `3.1416` `314.16e-2`
/// `0.31416E1` `34e1` `0x0.1E` `0xA23p-4` `0X1.921FB54442D18P+1`
pub fn numeral(input: &str) -> IResult<&str, Numeral> {
	todo!()
}

/// Parses string literals.
///
/// Literals can be one line or multiple lines. Multi-line literals
/// are delimited with brackets on both ends of the string.
pub fn literal_string(input: &str) -> IResult<&str, LiteralString> {
	todo!()
}
