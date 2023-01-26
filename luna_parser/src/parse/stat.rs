use nom::{IResult, bytes::complete::tag};

use crate::terminal::string::SEMICOLON;

pub fn semicolon(input: &str) -> IResult<&str, &str> {
	tag(SEMICOLON)(input)
}

pub fn varlist_explist(input: &str) -> IResult<&str, ()> {
	todo!()
}
