use luna_ast::types::{ElseStatementOption, ExpressionList, IfStatement, Statement, VariableList};
use nom::{branch::alt, bytes::complete::tag, IResult, multi::many_till};

use crate::terminal::{
	identifier,
	keyword::{kbreak, kdo, kelseif, kend, kgoto, kif, krepeat, kthen, kuntil, kwhile, kelse},
	string::SEMICOLON,
};

use super::{block, exp, functioncall, label};

fn semicolon(input: &str) -> IResult<&str, Statement> {
	let (input, _) = tag(SEMICOLON)(input)?;
	Ok((input, Statement::End))
}

fn fcall(input: &str) -> IResult<&str, Statement> {
	let (input, fcall) = functioncall(input)?;
	Ok((input, fcall.into()))
}

fn label_state(input: &str) -> IResult<&str, Statement> {
	let (input, lab) = label(input)?;
	Ok((input, lab.into()))
}

fn break_state(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kbreak(input)?;
	Ok((input, Statement::Break))
}

fn varlist_explist(input: &str) -> IResult<&str, Statement> {
	Ok((input, Statement::Definition(VariableList, ExpressionList)))
}

fn goto_name(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kgoto(input)?;
	let (input, id) = identifier(input)?;
	Ok((input, Statement::Goto(id)))
}

fn do_block(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kdo(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kend(input)?;
	Ok((input, Statement::Do(Box::new(bl))))
}

fn while_do(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kwhile(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = kdo(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kend(input)?;
	Ok((input, Statement::While(exp, bl)))
}

fn repeat_until(input: &str) -> IResult<&str, Statement> {
	let (input, _) = krepeat(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = kuntil(input)?;
	let (input, exp) = exp(input)?;
	Ok((input, Statement::RepeatUntil(bl, exp)))
}

fn end_tree(input: &str) -> IResult<&str, IfStatement> {
	let (input, _) = kend(input)?;
	Ok((input, IfStatement::End))
}

fn else_tree(input: &str) -> IResult<&str, IfStatement> {
	let (input, _) = alt((
		kelse,
		kend
	))(input)?;
	let (input, block) = block(input)?;
	let (input, _) = kend(input)?;
	Ok((input, IfStatement::Else(block)))
}

fn else_if_tree(input: &str) -> IResult<&str, IfStatement> {
	let (input, _) = kelseif(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = kthen(input)?;
	let (input, block) = block(input)?;
	Ok((input, IfStatement::ElseIf(exp, block)))
}

fn if_tree(input: &str) -> IResult<&str, Statement> {
	let (input, _) = kif(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = kthen(input)?;
	let (input, block) = block(input)?;

	let (input, tree) = many_till(else_if_tree, else_tree)(input)?;


	Ok((input, Statement::IfStatement()))
}

pub fn stat(input: &str) -> IResult<&str, Statement> {
	alt((
		semicolon,
		varlist_explist,
		fcall,
		label_state,
		break_state,
		goto_name,
		do_block,
		while_do,
		repeat_until,
	))(input)
}
