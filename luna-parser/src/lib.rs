/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

/// Abbreviated parser result type
pub(crate) type IRes<'a, O> = IResult<In<'a>, O>;

use combinator::list;
use luna_ast::{Block, Chunk, ReturnStatement};
use nom::{
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

use combinator::{ws0, wschar, wstag};

mod combinator;
pub use nom::error;
mod parse;
pub mod terminal;

#[cfg(test)]
mod test;

pub fn chunk(input: In) -> Result<Chunk, nom::error::Error<In>> {
	dbg!(input);

	all_consuming(ws0(block).map(Chunk))
		.parse(input)
		.finish()
		// If there's any remaining input, there's a problem
		.map(|(_, chunk)| chunk)
	// Convert nom's error type into our error type
	// .map_err(|e| error::Error::from_error_kind(e.input, e.code))
}

pub(crate) fn block(input: In) -> IRes<Block> {
	dbg!(input);
	pair(many0(ws0(stat)), opt(ws0(retstat)))
		.map(|(stlist, oret)| Block { stlist, oret })
		.parse(input)
}

pub(crate) fn retstat(input: In) -> IRes<ReturnStatement> {
	dbg!(input);
	delimited(
		wstag(KRETURN),
		opt(list(wschar(COMMA), exp)),
		opt(wschar(SEMICOLON)),
	)
	.map(|oelist| ReturnStatement { oelist })
	.parse(input)
}
