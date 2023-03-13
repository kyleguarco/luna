use luna_ast::types::{Block, IfStatement, Statement};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, opt, value},
	multi::many_till,
	sequence::{delimited, pair, preceded, separated_pair, tuple},
	IResult,
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

fn else_tree(input: &str) -> IResult<&str, IfStatement> {
	if let Ok((input, _)) = keyword::<&str, nom::error::Error<&str>>(End)(input) {
		return Ok((input, IfStatement::End));
	}

	let (input, block) = delimited(keyword(Else), block, keyword(End))(input)?;
	Ok((input, IfStatement::Else(block)))
}

fn else_if_tree(input: &str) -> IResult<&str, IfStatement> {
	let (input, exp) = preceded(keyword(ElseIf), exp)(input)?;
	let (input, block) = preceded(keyword(Then), block)(input)?;
	Ok((input, IfStatement::ElseIf(exp, block)))
}

fn if_tree(input: &str) -> IResultStat {
	let (input, exp) = preceded(keyword(If), exp)(input)?;
	let (input, block) = preceded(keyword(Then), block)(input)?;
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

fn bif(input: &str) {
	let p_if = preceded(keyword(If), exp);
	let p_elif = preceded(keyword(ElseIf), exp);
	let p_then = preceded(keyword(Then), block);
}

pub fn stat(input: &str) -> IResultStat {
	fn do_block(input: &str) -> IResult<&str, Block> {
		delimited(keyword(Do), block, keyword(End))(input)
	}

	alt((
		value(Statement::End, tag(SEMICOLON)),
		// This comes before 'varlist' to detect the presence of 'local'
		map(
			preceded(keyword(Local), pair(attnamelist, opt(explist))),
			Statement::LocalDefinitionWithAttribute,
		),
		map(pair(varlist, explist), Statement::Definition),
		map(functioncall, Statement::FunctionCall),
		map(label, Statement::Label),
		value(Statement::Break, keyword(Break)),
		map(preceded(keyword(Goto), identifier), Statement::Goto),
		map(do_block, |bl| Statement::Do(Box::new(bl))),
		map(
			pair(preceded(keyword(While), exp), do_block),
			Statement::While,
		),
		map(
			pair(
				preceded(keyword(Repeat), block),
				preceded(keyword(Until), exp),
			),
			|(bl, ex)| Statement::RepeatUntil(bl, ex),
		),
		// if_tree,
		map(
			tuple((
				preceded(keyword(For), identifier),
				tuple((exp, exp, opt(exp))),
				do_block,
			)),
			Statement::ForExpression,
		),
		map(
			pair(
				preceded(keyword(For), separated_pair(namelist, keyword(In), explist)),
				do_block,
			),
			Statement::ForList,
		),
		map(
			preceded(keyword(Function), pair(funcname, funcbody)),
			Statement::FunctionDefinition,
		),
		map(
			preceded(
				keyword(Local),
				preceded(keyword(Function), pair(identifier, funcbody)),
			),
			Statement::LocalFunctionDefinition,
		),
	))(input)
}
