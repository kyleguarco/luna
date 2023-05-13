mod combinator;
mod parse;
mod terminal;

/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

/// Abbreviated parser result type
pub(crate) type IRes<'a, O> = IResult<In<'a>, O>;

use luna_ast::{Chunk, Block};
use nom::IResult;

pub fn chunk(input: In) -> IRes<Chunk> {
	todo!()
}

pub fn block(input: In) -> IRes<Block> {
	todo!()
}
