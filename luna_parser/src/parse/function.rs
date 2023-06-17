//! # Function Structure Parsers

use luna_ast::function::{
	Arguments, AsFunction, AsMethod, FunctionBody, FunctionCall, FunctionName, ParameterList,
	VarArgs,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{opt, value},
	sequence::{pair, preceded, separated_pair},
	Parser,
};

use crate::{
	block,
	combinator::paren,
	terminal::{
		literal_string, name, name_list,
		string::{COLON, COMMA, TRIPLEDOT},
	},
	IRes, In,
};

use super::{
	expression::{explist, prefixexp},
	table::tableconstructor,
};

pub(super) fn var_args(input: In) -> IRes<VarArgs> {
	dbg!(input);
	value(VarArgs, tag(TRIPLEDOT)).parse(input)
}

pub fn funcname(input: In) -> IRes<FunctionName> {
	dbg!(input);
	pair(name_list, opt(preceded(tchar(COLON), name)))
		.map(|(nlist, objname)| FunctionName { nlist, objname })
		.parse(input)
}

fn as_func(input: In) -> IRes<AsFunction> {
	dbg!(input);
	pair(prefixexp, args)
		.map(|(pexp, argu)| AsFunction { pexp, argu })
		.parse(input)
}

fn as_method(input: In) -> IRes<AsMethod> {
	dbg!(input);
	separated_pair(prefixexp, tchar(COLON), pair(name, args))
		.map(|(pexp, (name, argu))| AsMethod { pexp, name, argu })
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
	alt((
		as_func.map(FunctionCall::from),
		as_method.map(FunctionCall::from),
	))
	.parse(input)
}

fn args(input: In) -> IRes<Arguments> {
	dbg!(input);
	alt((
		paren(opt(explist)).map(Arguments::from),
		tableconstructor.map(Arguments::from),
		literal_string.map(Arguments::from),
	))
	.parse(input)
}

pub fn parlist(input: In) -> IRes<ParameterList> {
	dbg!(input);
	alt((
		separated_pair(name_list, tchar(COMMA), var_args).map(ParameterList::from),
		var_args.map(ParameterList::from),
	))
	.parse(input)
}
