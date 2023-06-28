//! # Function Structure Parsers

use luna_ast::function::{
	Arguments, FunctionBody, FunctionCall, FunctionName, ParameterList, VarArgs,
};
use nom::{
	branch::alt,
	combinator::{opt, value},
	multi::many0,
	sequence::{pair, preceded, tuple},
	Parser,
};

use crate::{
	block,
	combinator::{list, paren, wschar, wstag},
	parse::affix::{call, prefix, suffix},
	terminal::{
		literal_string, name, namelist,
		string::{COLON, COMMA, DOT, TRIPLEDOT},
	},
	IRes, In,
};

use super::{expression::explist, table::tableconstructor};

pub(super) fn varargs(input: In) -> IRes<VarArgs> {
	dbg!(input);
	value(VarArgs, wstag(TRIPLEDOT)).parse(input)
}

pub fn funcname(input: In) -> IRes<FunctionName> {
	dbg!(input);
	pair(list(wschar(DOT), name), opt(preceded(wschar(COLON), name)))
		.map(|(nlist, objname)| FunctionName { nlist, objname })
		.parse(input)
}

pub fn funcbody(input: In) -> IRes<FunctionBody> {
	dbg!(input);
	pair(paren(opt(parlist)), block)
		.map(|(oplist, bl)| FunctionBody { oplist, bl })
		.parse(input)
}

pub fn functioncall(input: In) -> IRes<FunctionCall> {
	dbg!(input);
	tuple((prefix, many0(suffix), call))
		.map(|(pfix, slist, call)| FunctionCall { pfix, slist, call })
		.parse(input)
}

pub fn args(input: In) -> IRes<Arguments> {
	dbg!(input);
	alt((
		paren(opt(explist)).map(Arguments::from),
		tableconstructor.map(Arguments::from),
		literal_string.map(Arguments::from),
	))
	.parse(input)
}

pub fn parlist(input: In) -> IRes<ParameterList> {
	use ParameterList::*;

	dbg!(input);
	alt((
		namelist
			.and(opt(preceded(wschar(COMMA), varargs)))
			.map(|(nlist, vargs)| match vargs {
				Some(_) => NameListWithVarArgs(nlist),
				None => NameList(nlist),
			}),
		varargs.map(VarArgs),
	))
	.parse(input)
}
