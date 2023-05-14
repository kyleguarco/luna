/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

/// Abbreviated parser result type
pub(crate) type IRes<'a, O> = IResult<In<'a>, O>;

use combinator::list;
use error::ParseError;
use luna_ast::{Block, Chunk, ReturnStatement};
use nom::{
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{all_consuming, opt},
	multi::many0,
	sequence::{delimited, pair},
	Finish, IResult, Parser,
};
use parse::{expression::exp, statement::stat};
use terminal::{
	keyword::KRETURN,
	string::{COMMA, SEMICOLON},
};

mod combinator;
pub mod error;
mod parse;
pub mod terminal;

pub fn chunk(input: In) -> Result<Chunk, ParseError<&str>> {
	all_consuming(block.map(Chunk))
		.parse(input)
		.finish()
		// If there's any remaining input, there's a problem
		.map(|(_, chunk)| chunk)
		.map_err(ParseError::from)
}

pub(crate) fn block(input: In) -> IRes<Block> {
	pair(many0(stat), opt(return_stat))
		.map(|(stlist, oret)| Block { stlist, oret })
		.parse(input)
}

pub(crate) fn return_stat(input: In) -> IRes<ReturnStatement> {
	delimited(
		tag(KRETURN),
		opt(list(tchar(COMMA), exp)),
		opt(tchar(SEMICOLON)),
	)
	.map(|oelist| ReturnStatement { oelist })
	.parse(input)
}
