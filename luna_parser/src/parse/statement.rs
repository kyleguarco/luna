use luna_ast::statement::{
	Definition, ForExpression, ForList, FunctionDefinition, IfBlock, IfTree, Label,
	LocalDefinitionWithAttribute, LocalFunctionDefinition, RepeatUntil, Statement, While,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{opt, value},
	multi::many0,
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
	Parser,
};

use crate::{
	block,
	combinator::assign,
	terminal::{
		keyword::{
			KBREAK, KDO, KELSE, KELSEIF, KEND, KFOR, KFUNCTION, KGOTO, KIF, KIN, KLOCAL, KREPEAT,
			KTHEN, KUNTIL, KWHILE,
		},
		name, name_list,
		string::{COMMA, DOUBLECOLON, EQUALS, SEMICOLON},
	},
	IRes, In,
};

use super::{
	attribute::attnamelist,
	expression::{exp, explist},
	function::{funcbody, funcname, functioncall},
	variable::varlist,
};

pub fn label(input: In) -> IRes<Label> {
	dbg!(input);
	delimited(tag(DOUBLECOLON), name, tag(DOUBLECOLON))
		.map(|name| Label(name))
		.parse(input)
}

fn if_block(keyword: In) -> impl FnMut(In) -> IRes<IfBlock> + '_ {
	move |input: In| {
		preceded(tag(keyword), separated_pair(exp, tag(KTHEN), block))
			.map(|(cond, bl)| IfBlock { cond, bl })
			.parse(input)
	}
}

fn if_tree(input: In) -> IRes<IfTree> {
	dbg!(input);
	tuple((
		if_block(KIF),
		many0(if_block(KELSEIF)),
		opt(preceded(tag(KELSE), block)),
	))
	.map(|(initial, elseifs, otherwise)| IfTree { initial, elseifs, otherwise })
	.parse(input)
}

fn for_exp(input: In) -> IRes<ForExpression> {
	dbg!(input);
	let parse_exp = tuple((
		exp,
		preceded(tchar(COMMA), exp),
		opt(preceded(tchar(COMMA), exp)),
	));
	let lhs = preceded(tag(KFOR), name);
	let rhs = pair(
		terminated(parse_exp, tag(KDO)),
		terminated(block, tag(KEND)),
	);
	assign(lhs, rhs)
		.map(|(name, ((start, stop, step), bl))| ForExpression {
			name,
			range: start..=stop,
			step,
			bl,
		})
		.parse(input)
}

fn for_list(input: In) -> IRes<ForList> {
	dbg!(input);
	tuple((
		preceded(tag(KFOR), name_list),
		delimited(tag(KIN), explist, tag(KDO)),
		terminated(block, tag(KEND)),
	))
	.map(|(nlist, elist, bl)| ForList { nlist, elist, bl })
	.parse(input)
}

fn pwhile(input: In) -> IRes<While> {
	dbg!(input);
	delimited(tag(KWHILE), separated_pair(exp, tag(KDO), block), tag(KEND))
		.map(|(cond, bl)| While { cond, bl })
		.parse(input)
}

fn repeat_until(input: In) -> IRes<RepeatUntil> {
	dbg!(input);
	preceded(tag(KREPEAT), separated_pair(block, tag(KUNTIL), exp))
		.map(|(bl, cond)| RepeatUntil { cond, bl })
		.parse(input)
}

fn definition(input: In) -> IRes<Definition> {
	dbg!(input);
	assign(varlist, explist)
		.map(|(vlist, elist)| Definition { vlist, elist })
		.parse(input)
}

pub fn functiondef(input: In) -> IRes<FunctionDefinition> {
	dbg!(input);
	pair(preceded(tag(KFUNCTION), funcname), funcbody)
		.map(|(fname, fbody)| FunctionDefinition { fname, fbody })
		.parse(input)
}

fn local_func_def(input: In) -> IRes<LocalFunctionDefinition> {
	dbg!(input);
	preceded(tag(KLOCAL), pair(preceded(tag(KFUNCTION), name), funcbody))
		.map(|(name, fbody)| LocalFunctionDefinition { name, fbody })
		.parse(input)
}

fn local_def_attr(input: In) -> IRes<LocalDefinitionWithAttribute> {
	dbg!(input);
	preceded(
		tag(KLOCAL),
		pair(attnamelist, opt(preceded(tchar(EQUALS), explist))),
	)
	.map(|(atlist, oelist)| LocalDefinitionWithAttribute { atlist, oelist })
	.parse(input)
}

pub fn stat(input: In) -> IRes<Statement> {
	use Statement::*;

	dbg!(input);
	alt((
		value(Statement::End, tchar(SEMICOLON)),
		definition.map(Definition),
		functioncall.map(FunctionCall),
		label.map(Label),
		value(Break, tag(KBREAK)),
		preceded(tag(KGOTO), name).map(Goto),
		delimited(tag(KWHILE), block, tag(KEND))
			.map(Box::new)
			.map(Do),
		pwhile.map(While),
		repeat_until.map(RepeatUntil),
		if_tree.map(IfTree),
		for_exp.map(ForExpression),
		for_list.map(ForList),
		functiondef.map(FunctionDefinition),
		local_func_def.map(LocalFunctionDefinition),
		local_def_attr.map(LocalDefinitionWithAttribute),
	))
	.parse(input)
}
