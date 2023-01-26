use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, alphanumeric1},
	combinator::recognize,
	multi::many0_count,
	sequence::pair,
	IResult,
};

pub(crate) mod string;
pub(crate) mod keyword;

/// Parses identifiers (which share naming conventions with Rust).
/// Lua refers to identifiers as "Names" in their documentation.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#rust-style-identifiers
pub fn identifier(input: &str) -> IResult<&str, &str> {
	recognize(pair(
		alt((alpha1, tag("_"))),
		many0_count(alt((alphanumeric1, tag("_")))),
	))(input)
}

// https://github.com/rust-bakery/nom/blob/main/examples/string.rs
