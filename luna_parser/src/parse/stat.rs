use nom::{IResult, bytes::complete::tag};

use crate::terminal::{string::SEMICOLON, keyword::{kgoto, kdo, kend}, identifier};

use super::block;

pub fn semicolon(input: &str) -> IResult<&str, &str> {
	tag(SEMICOLON)(input)
}

pub fn varlist_explist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn goto_name(input: &str) -> IResult<&str, &str> {
	let (_, input) = kgoto(input)?;
	identifier(input)
}

pub fn do_block(input: &str) -> IResult<&str, &str> {
	let (_, input) = kdo(input)?;
	let (_, input) = block(input)?;
	kend(input)
}
