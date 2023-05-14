use luna_ast::statement::{
	Definition, ForExpression, ForList, FunctionDefinition, IfBlock, IfTree, Label,
	LocalDefinitionWithAttribute, LocalFunctionDefinition, RepeatUntil, Statement, While,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::value,
	sequence::{delimited, preceded, separated_pair},
	Parser,
};

use crate::{
	block,
	terminal::{
		keyword::{KBREAK, KDO, KEND, KGOTO, KREPEAT, KUNTIL, KWHILE},
		name,
		string::{DOUBLECOLON, SEMICOLON},
	},
	IRes, In,
};

use super::{expression::exp, function::func_call};

pub fn label(input: In) -> IRes<Label> {
	delimited(tag(DOUBLECOLON), name, tag(DOUBLECOLON))
		.map(|name| Label(name))
		.parse(input)
}

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
	delimited(tag(KWHILE), separated_pair(exp, tag(KDO), block), tag(KEND))
		.map(|(cond, bl)| While { cond, bl })
		.parse(input)
}

pub fn repeat_until(input: In) -> IRes<RepeatUntil> {
	preceded(tag(KREPEAT), separated_pair(block, tag(KUNTIL), exp))
		.map(|(bl, cond)| RepeatUntil { cond, bl })
		.parse(input)
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
