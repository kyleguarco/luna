use luna_ast::{
	statement::{
		Assignment, ForExpression, ForList, IfBlock, IfTree, Label, LocalDefinitionWithAttribute,
		LocalFunctionDefinition, NamedFunctionDefinition, RepeatUntil, Statement, While,
	},
	Block,
};
use nom::{
	branch::alt,
	combinator::{opt, value},
	multi::many0,
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
	Parser,
};

use crate::{
	block,
	combinator::{assign, wschar, wstag},
	terminal::{
		keyword::{
			KBREAK, KDO, KELSE, KELSEIF, KEND, KFOR, KFUNCTION, KGOTO, KIF, KIN, KLOCAL, KREPEAT,
			KTHEN, KUNTIL, KWHILE,
		},
		name, namelist,
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
	delimited(wstag(DOUBLECOLON), name, wstag(DOUBLECOLON))
		.map(Label)
		.parse(input)
}

fn if_block(keyword: In) -> impl FnMut(In) -> IRes<IfBlock> + '_ {
	move |input: In| {
		preceded(wstag(keyword), separated_pair(exp, wstag(KTHEN), block))
			.map(|(cond, bl)| IfBlock { cond, bl })
			.parse(input)
	}
}

fn if_tree(input: In) -> IRes<IfTree> {
	dbg!(input);
	terminated(
		tuple((
			if_block(KIF),
			many0(if_block(KELSEIF)),
			opt(preceded(wstag(KELSE), block)),
		)),
		wstag(KEND),
	)
	.map(|(initial, elseifs, otherwise)| IfTree { initial, elseifs, otherwise })
	.parse(input)
}

fn for_exp(input: In) -> IRes<ForExpression> {
	dbg!(input);
	let parse_exp = tuple((
		exp,
		preceded(wschar(COMMA), exp),
		opt(preceded(wschar(COMMA), exp)),
	));
	let lhs = preceded(wstag(KFOR), name);
	let rhs = pair(
		terminated(parse_exp, wstag(KDO)),
		terminated(block, wstag(KEND)),
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
		preceded(wstag(KFOR), namelist),
		delimited(wstag(KIN), explist, wstag(KDO)),
		terminated(block, wstag(KEND)),
	))
	.map(|(nlist, elist, bl)| ForList { nlist, elist, bl })
	.parse(input)
}

fn doblk(input: In) -> IRes<Block> {
	delimited(wstag(KWHILE), block, wstag(KEND)).parse(input)
}

fn whileblk(input: In) -> IRes<While> {
	dbg!(input);
	delimited(
		wstag(KWHILE),
		separated_pair(exp, wstag(KDO), block),
		wstag(KEND),
	)
	.map(|(cond, bl)| While { cond, bl })
	.parse(input)
}

fn repeat_until(input: In) -> IRes<RepeatUntil> {
	dbg!(input);
	preceded(wstag(KREPEAT), separated_pair(block, wstag(KUNTIL), exp))
		.map(|(bl, cond)| RepeatUntil { cond, bl })
		.parse(input)
}

fn assignment(input: In) -> IRes<Assignment> {
	dbg!(input);
	assign(varlist, explist)
		.map(|(vlist, elist)| Assignment { vlist, elist })
		.parse(input)
}

fn named_functiondef(input: In) -> IRes<NamedFunctionDefinition> {
	dbg!(input);
	pair(preceded(wstag(KFUNCTION), funcname), funcbody)
		.map(|(fname, fbody)| NamedFunctionDefinition { fname, fbody })
		.parse(input)
}

fn local_func_def(input: In) -> IRes<LocalFunctionDefinition> {
	dbg!(input);
	preceded(
		wstag(KLOCAL),
		pair(preceded(wstag(KFUNCTION), name), funcbody),
	)
	.map(|(name, fbody)| LocalFunctionDefinition { name, fbody })
	.parse(input)
}

fn local_def_attr(input: In) -> IRes<LocalDefinitionWithAttribute> {
	dbg!(input);
	preceded(
		wstag(KLOCAL),
		pair(attnamelist, opt(preceded(wschar(EQUALS), explist))),
	)
	.map(|(atlist, oelist)| LocalDefinitionWithAttribute { atlist, oelist })
	.parse(input)
}

pub fn stat(input: In) -> IRes<Statement> {
	dbg!(input);
	use Statement::*;

	alt((
		value(Statement::End, wschar(SEMICOLON)),
		assignment.map(Assignment),
		functioncall.map(FunctionCall),
		label.map(Label),
		value(Break, wstag(KBREAK)),
		preceded(wstag(KGOTO), name).map(Goto),
		doblk.map(Box::new).map(Do),
		whileblk.map(While),
		repeat_until.map(RepeatUntil),
		if_tree.map(IfTree),
		for_exp.map(ForExpression),
		for_list.map(ForList),
		named_functiondef.map(FunctionDefinition),
		local_func_def.map(LocalFunctionDefinition),
		local_def_attr.map(LocalDefinitionWithAttribute),
	))
	.parse(input)
}
