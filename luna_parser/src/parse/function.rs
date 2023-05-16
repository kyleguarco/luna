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
	terminal::{
		literal_string, name, name_list,
		string::{COLON, COMMA, TRIPLEDOT},
	},
	IRes, In,
};

use super::{
	expression::{exp_list, prefix_exp},
	table::table_cons,
};

pub fn var_args(input: In) -> IRes<VarArgs> {
	dbg!(input);
	value(VarArgs, tag(TRIPLEDOT)).parse(input)
}

pub fn func_name(input: In) -> IRes<FunctionName> {
	dbg!(input);
	pair(name_list, opt(preceded(tchar(COLON), name)))
		.map(|(nlist, objname)| FunctionName { nlist, objname })
		.parse(input)
}

fn as_func(input: In) -> IRes<AsFunction> {
	dbg!(input);
	pair(prefix_exp, args)
		.map(|(pexp, argu)| AsFunction { pexp, argu })
		.parse(input)
}

fn as_method(input: In) -> IRes<AsMethod> {
	dbg!(input);
	separated_pair(prefix_exp, tchar(COLON), pair(name, args))
		.map(|(pexp, (name, argu))| AsMethod { pexp, name, argu })
		.parse(input)
}

pub fn func_body(input: In) -> IRes<FunctionBody> {
	dbg!(input);
	pair(opt(par_list), block)
		.map(|(oplist, bl)| FunctionBody { oplist, bl })
		.parse(input)
}

pub fn func_call(input: In) -> IRes<FunctionCall> {
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
		opt(exp_list).map(Arguments::from),
		table_cons.map(Arguments::from),
		literal_string.map(Arguments::from),
	))
	.parse(input)
}

pub fn par_list(input: In) -> IRes<ParameterList> {
	dbg!(input);
	alt((
		separated_pair(name_list, tchar(COMMA), var_args).map(ParameterList::from),
		var_args.map(ParameterList::from),
	))
	.parse(input)
}
