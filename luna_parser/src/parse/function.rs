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
	multi::separated_list1,
	sequence::{pair, preceded, separated_pair},
	Parser,
};

use crate::{
	block,
	combinator::list,
	terminal::{
		literal_string, name,
		string::{COLON, COMMA, TRIPLEDOT},
	},
	IRes, In,
};

use super::{
	expression::{exp, prefix_exp},
	table::table_cons,
};

pub fn var_args(input: In) -> IRes<VarArgs> {
	value(VarArgs, tag(TRIPLEDOT)).parse(input)
}

pub fn func_name(input: In) -> IRes<FunctionName> {
	pair(list(tchar(COMMA), name), opt(preceded(tchar(COLON), name)))
		.map(|(nlist, objname)| FunctionName { nlist, objname })
		.parse(input)
}

pub fn as_func(input: In) -> IRes<AsFunction> {
	pair(prefix_exp, args)
		.map(|(pexp, argu)| AsFunction { pexp, argu })
		.parse(input)
}

pub fn as_method(input: In) -> IRes<AsMethod> {
	separated_pair(prefix_exp, tchar(COLON), pair(name, args))
		.map(|(pexp, (name, argu))| AsMethod { pexp, name, argu })
		.parse(input)
}

pub fn func_body(input: In) -> IRes<FunctionBody> {
	pair(opt(par_list), block)
		.map(|(oplist, bl)| FunctionBody { oplist, bl })
		.parse(input)
}

pub fn func_call(input: In) -> IRes<FunctionCall> {
	alt((
		as_func.map(FunctionCall::from),
		as_method.map(FunctionCall::from),
	))
	.parse(input)
}

pub fn args(input: In) -> IRes<Arguments> {
	alt((
		opt(list(tchar(COMMA), exp)).map(Arguments::from),
		table_cons.map(Arguments::from),
		literal_string.map(Arguments::from),
	))
	.parse(input)
}

pub fn par_list(input: In) -> IRes<ParameterList> {
	alt((
		separated_pair(list(tchar(COMMA), name), tchar(COMMA), var_args).map(ParameterList::from),
		var_args.map(ParameterList::from),
	))
	.parse(input)
}
