/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

/// Abbreviated parser result type
pub(crate) type IRes<'a, O> = IResult<In<'a>, O>;

use combinator::list;
use luna_ast::{Block, Chunk, ReturnStatement};
use nom::{
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{all_consuming, opt},
	error::ParseError,
	multi::many0,
	sequence::{delimited, pair},
	Finish, IResult, Parser,
};
use parse::{expression::exp, statement::stat};
use terminal::{
	keyword::KRETURN,
	string::{COMMA, SEMICOLON},
};

use crate::combinator::ws0;

mod combinator;
pub mod error;
mod parse;
pub mod terminal;

#[cfg(test)]
mod test;

pub fn chunk(input: In) -> Result<Chunk, error::Error<In>> {
	dbg!(input);
	all_consuming(ws0(block).map(Chunk))
		.parse(input)
		.finish()
		// If there's any remaining input, there's a problem
		.map(|(_, chunk)| chunk)
		// Convert nom's error type into our error type
		.map_err(|e| error::Error::from_error_kind(e.input, e.code))
}

pub(crate) fn block(input: In) -> IRes<Block> {
	dbg!(input);
	// TODO! This infinitely loops because many0 doesn't exit. Weird...
	pair(many0(stat), opt(return_stat))
		.map(|(stlist, oret)| Block { stlist, oret })
		.parse(input)
}

pub(crate) fn return_stat(input: In) -> IRes<ReturnStatement> {
	dbg!(input);
	delimited(
		tag(KRETURN),
		opt(list(tchar(COMMA), exp)),
		opt(tchar(SEMICOLON)),
	)
	.map(|oelist| ReturnStatement { oelist })
	.parse(input)
}
