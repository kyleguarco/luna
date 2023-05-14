use luna_ast::statement::{
	Definition, ForExpression, ForList, FunctionDefinition, IfBlock, IfTree,
	LocalDefinitionWithAttribute, LocalFunctionDefinition, RepeatUntil, Statement, While,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::value,
	sequence::{delimited, preceded},
	Parser,
};

use crate::{
	block,
	terminal::{
		keyword::{KBREAK, KEND, KGOTO, KWHILE},
		name,
		string::SEMICOLON,
	},
	IRes, In,
};

use super::{function::func_call, misc::label};

pub fn if_block(input: In) -> IRes<IfBlock> {
	todo!()
}

pub fn if_tree(input: In) -> IRes<IfTree> {
	todo!()
}

pub fn for_exp(input: In) -> IRes<ForExpression> {
	todo!()
}

pub fn for_list(input: In) -> IRes<ForList> {
	todo!()
}

pub fn pwhile(input: In) -> IRes<While> {
	todo!()
}

pub fn repeat_until(input: In) -> IRes<RepeatUntil> {
	todo!()
}

pub fn definition(input: In) -> IRes<Definition> {
	todo!()
}

pub fn func_def(input: In) -> IRes<FunctionDefinition> {
	todo!()
}

pub fn local_func_def(input: In) -> IRes<LocalFunctionDefinition> {
	todo!()
}

pub fn local_func_def_attr(input: In) -> IRes<LocalDefinitionWithAttribute> {
	todo!()
}

pub fn stat(input: In) -> IRes<Statement> {
	alt((
		value(Statement::End, tchar(SEMICOLON)),
		definition.map(Statement::from),
		func_call.map(Statement::from),
		label.map(Statement::from),
		value(Statement::Break, tag(KBREAK)),
		preceded(tag(KGOTO), name).map(Statement::Goto),
		delimited(tag(KWHILE), block, tag(KEND))
			.map(Box::new)
			.map(Statement::Do),
		pwhile.map(Statement::from),
		repeat_until.map(Statement::from),
		if_tree.map(Statement::from),
		for_exp.map(Statement::from),
		for_list.map(Statement::from),
		func_def.map(Statement::from),
		local_func_def.map(Statement::from),
		local_func_def_attr.map(Statement::from),
	))
	.parse(input)
}
