use luna_ast::types::{IfStatement, Statement};
use nom::{
	branch::alt, bytes::complete::tag, combinator::opt, multi::many_till, sequence::tuple, IResult,
};

use crate::terminal::{
	identifier,
	keyword::{keyword, Keyword::*},
	string::SEMICOLON,
};

use super::{
	attnamelist, block, exp, explist, funcbody, funcname, functioncall, label, namelist, varlist,
};

type IResultStat<'a> = IResult<&'a str, Statement>;

fn semicolon(input: &str) -> IResultStat {
	let (input, _) = tag(SEMICOLON)(input)?;
	Ok((input, Statement::End))
}

fn fcall(input: &str) -> IResultStat {
	let (input, fcall) = functioncall(input)?;
	Ok((input, fcall.into()))
}

fn label_state(input: &str) -> IResultStat {
	let (input, lab) = label(input)?;
	Ok((input, lab.into()))
}

fn break_state(input: &str) -> IResultStat {
	let (input, _) = keyword(Break)(input)?;
	Ok((input, Statement::Break))
}

fn varlist_explist(input: &str) -> IResultStat {
	let (input, vlist) = varlist(input)?;
	let (input, elist) = explist(input)?;

	Ok((input, Statement::Definition(vlist, elist)))
}

fn goto_name(input: &str) -> IResultStat {
	let (input, _) = keyword(Goto)(input)?;
	let (input, id) = identifier(input)?;
	Ok((input, Statement::Goto(id)))
}

fn do_block(input: &str) -> IResultStat {
	let (input, _) = keyword(Do)(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(End)(input)?;
	Ok((input, Statement::Do(Box::new(bl))))
}

fn while_do(input: &str) -> IResultStat {
	let (input, _) = keyword(While)(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = keyword(Do)(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(End)(input)?;
	Ok((input, Statement::While(exp, bl)))
}

fn repeat_until(input: &str) -> IResultStat {
	let (input, _) = keyword(Repeat)(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(Until)(input)?;
	let (input, exp) = exp(input)?;
	Ok((input, Statement::RepeatUntil(bl, exp)))
}

fn else_tree(input: &str) -> IResult<&str, IfStatement> {
	if let Ok((input, _)) = keyword::<&str, nom::error::Error<&str>>(End)(input) {
		return Ok((input, IfStatement::End));
	}

	let (input, _) = keyword(Else)(input)?;
	let (input, block) = block(input)?;
	let (input, _) = keyword(End)(input)?;
	Ok((input, IfStatement::Else(block)))
}

fn else_if_tree(input: &str) -> IResult<&str, IfStatement> {
	let (input, _) = keyword(ElseIf)(input)?;
	let (input, exp) = exp(input)?;
	let (input, _) = keyword(Then)(input)?;
	let (input, block) = block(input)?;
	Ok((input, IfStatement::ElseIf(exp, block)))
}

fn if_tree(input: &str) -> IResultStat {
	let (input, _) = keyword(If)(input)?;

	let (input, exp) = exp(input)?;
	let (input, _) = keyword(Then)(input)?;
	let (input, block) = block(input)?;
	let initial = IfStatement::If(exp, block);

	let (input, (tree, belse)) = many_till(else_if_tree, else_tree)(input)?;

	Ok((
		input,
		Statement::IfStatement {
			initial,
			belse,
			tree,
		},
	))
}

fn for_exp(input: &str) -> IResultStat {
	let (input, _) = keyword(For)(input)?;
	let (input, ident) = identifier(input)?;

	let (input, exps) = tuple((exp, exp, opt(exp)))(input)?;
	let (input, _) = keyword(Do)(input)?;
	let (input, block) = block(input)?;
	let (input, _) = keyword(End)(input)?;

	Ok((input, Statement::ForExpression(ident, exps, block)))
}

fn for_list(input: &str) -> IResultStat {
	let (input, _) = keyword(For)(input)?;

	let (input, ilist) = namelist(input)?;
	let (input, _) = keyword(In)(input)?;
	let (input, elist) = explist(input)?;

	let (input, _) = keyword(Do)(input)?;
	let (input, block) = block(input)?;
	let (input, _) = keyword(End)(input)?;

	Ok((input, Statement::ForList(ilist, elist, block)))
}

fn fdec(input: &str) -> IResultStat {
	let (input, _) = keyword(Function)(input)?;
	let (input, fident) = funcname(input)?;
	let (input, fblock) = funcbody(input)?;

	Ok((input, Statement::FunctionDefinition(fident, fblock)))
}

fn flocaldec(input: &str) -> IResultStat {
	let (input, _) = keyword(Local)(input)?;
	let (input, _) = keyword(Function)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, fblock) = funcbody(input)?;

	Ok((input, Statement::LocalFunctionDefinition(ident, fblock)))
}

fn local_attlist(input: &str) -> IResultStat {
	let (input, _) = keyword(Local)(input)?;
	let (input, alist) = attnamelist(input)?;
	let (input, elist) = opt(explist)(input)?;

	Ok((input, Statement::LocalDefinitionWithAttribute(alist, elist)))
}

pub fn stat(input: &str) -> IResultStat {
	alt((
		semicolon,
		// This comes before 'varlist' to detect the presence of 'local'
		local_attlist,
		varlist_explist,
		fcall,
		label_state,
		break_state,
		goto_name,
		do_block,
		while_do,
		repeat_until,
		if_tree,
		for_exp,
		for_list,
		fdec,
		flocaldec,
	))(input)
}
