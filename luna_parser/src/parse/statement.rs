use luna_ast::statement::{
	Definition, ForExpression, ForList, FunctionDefinition, IfBlock, IfTree, Label,
	LocalDefinitionWithAttribute, LocalFunctionDefinition, RepeatUntil, Statement, While,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{cond, eof, fail, opt, value},
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
	attribute::att_name_list,
	expression::{exp, exp_list},
	function::{func_body, func_call, func_name},
	variable::var_list,
};

pub fn label(input: In) -> IRes<Label> {
	dbg!(input);
	delimited(tag(DOUBLECOLON), name, tag(DOUBLECOLON))
		.map(|name| Label(name))
		.parse(input)
}

pub fn if_block(keyword: In) -> impl FnMut(In) -> IRes<IfBlock> + '_ {
	move |input: In| {
		preceded(tag(keyword), separated_pair(exp, tag(KTHEN), block))
			.map(|(cond, bl)| IfBlock { cond, bl })
			.parse(input)
	}
}

pub fn if_tree(input: In) -> IRes<IfTree> {
	dbg!(input);
	tuple((
		if_block(KIF),
		many0(if_block(KELSEIF)),
		opt(preceded(tag(KELSE), block)),
	))
	.map(|(initial, elseifs, otherwise)| IfTree { initial, elseifs, otherwise })
	.parse(input)
}

pub fn for_exp(input: In) -> IRes<ForExpression> {
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

pub fn for_list(input: In) -> IRes<ForList> {
	dbg!(input);
	tuple((
		preceded(tag(KFOR), name_list),
		delimited(tag(KIN), exp_list, tag(KDO)),
		terminated(block, tag(KEND)),
	))
	.map(|(nlist, elist, bl)| ForList { nlist, elist, bl })
	.parse(input)
}

pub fn pwhile(input: In) -> IRes<While> {
	dbg!(input);
	delimited(tag(KWHILE), separated_pair(exp, tag(KDO), block), tag(KEND))
		.map(|(cond, bl)| While { cond, bl })
		.parse(input)
}

pub fn repeat_until(input: In) -> IRes<RepeatUntil> {
	dbg!(input);
	preceded(tag(KREPEAT), separated_pair(block, tag(KUNTIL), exp))
		.map(|(bl, cond)| RepeatUntil { cond, bl })
		.parse(input)
}

pub fn definition(input: In) -> IRes<Definition> {
	dbg!(input);
	assign(var_list, exp_list)
		.map(|(vlist, elist)| Definition { vlist, elist })
		.parse(input)
}

pub fn func_def(input: In) -> IRes<FunctionDefinition> {
	dbg!(input);
	pair(preceded(tag(KFUNCTION), func_name), func_body)
		.map(|(fname, fbody)| FunctionDefinition { fname, fbody })
		.parse(input)
}

pub fn local_func_def(input: In) -> IRes<LocalFunctionDefinition> {
	dbg!(input);
	preceded(tag(KLOCAL), pair(preceded(tag(KFUNCTION), name), func_body))
		.map(|(name, fbody)| LocalFunctionDefinition { name, fbody })
		.parse(input)
}

pub fn local_def_attr(input: In) -> IRes<LocalDefinitionWithAttribute> {
	dbg!(input);
	preceded(
		tag(KLOCAL),
		pair(att_name_list, opt(preceded(tchar(EQUALS), exp_list))),
	)
	.map(|(atlist, oelist)| LocalDefinitionWithAttribute { atlist, oelist })
	.parse(input)
}

pub fn stat(input: In) -> IRes<Statement> {
	dbg!(input);

	// Since this combinator is usually called with `many0`, we need
	// to provide a default end statement to prevent infinite recursion.
	let (input, _) = cond(input.len() == 0, fail::<_, &str, _>)(input)?;

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
		local_def_attr.map(Statement::from),
	))
	.parse(input)
}
