use luna_ast::types::{Statement, ExpressionList, VariableList};
use nom::{bytes::complete::tag, IResult};

use crate::terminal::{
	identifier,
	keyword::{kdo, kend, kgoto, krepeat, kuntil, kwhile, kbreak},
	string::SEMICOLON,
};

use super::{block, exp, functioncall, label};

pub fn semicolon(input: &str) -> IResult<&str, Statement> {
	let (input, _) = tag(SEMICOLON)(input)?;
	Ok((input, Statement::End))
}

pub fn fcall(input: &str) -> IResult<&str, Statement> {
	let (input, fcall) = functioncall(input)?;
	Ok((input, fcall.into()))
}

pub fn label_state(input: &str) -> IResult<&str, Statement> {
	let (input, lab) = label(input)?;
	Ok((input, lab.into()))
}

pub fn break_state(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kbreak(input)?;
	Ok((input, Statement::Break))
}

pub fn varlist_explist(input: &str) -> IResult<&str, Statement> {
	Ok((input, Statement::Definition(VariableList, ExpressionList)))
}

pub fn goto_name(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kgoto(input)?;
	let (input, id) = identifier(input)?;
	Ok((input, Statement::Goto(id)))
}

pub fn do_block(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kdo(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kend(input)?;
	Ok((input, Statement::Do(Box::new(bl))))
}

pub fn while_do(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kwhile(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = kdo(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kend(input)?;
	Ok((input, Statement::While(exp, bl)))
}

pub fn repeat_until(input: &str) -> IResult<&str, Statement> {
	let (input, _) = krepeat(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kuntil(input)?;
	let (input, exp) = exp(input)?;
	Ok((input, Statement::RepeatUntil(bl, exp)))
}
