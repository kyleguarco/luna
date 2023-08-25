//! # Function Structure Parsers

use luna_ast::{
	affix::Suffix,
	expression::AnonFunctionDefinition,
	function::{Arguments, FunctionBody, FunctionCall, FunctionName, ParameterList, VarArgs},
};
use nom::{
	branch::alt,
	combinator::{opt, value, verify},
	sequence::{pair, preceded, terminated},
	Parser,
};

use crate::{
	block,
	combinator::{list, paren, wschar, wstag},
	parse::affix::affix,
	terminal::{
		keyword::{KEND, KFUNCTION},
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
	list(wschar(DOT), name)
		.and(opt(preceded(wschar(COLON), name)))
		.map(|(nlist, objname)| FunctionName { nlist, objname })
		.parse(input)
}

pub fn funcbody(input: In) -> IRes<FunctionBody> {
	dbg!(input);
	terminated(pair(paren(opt(parlist)), block), wstag(KEND))
		.map(|(oplist, bl)| FunctionBody { oplist, bl })
		.parse(input)
}

pub fn functiondef(input: In) -> IRes<AnonFunctionDefinition> {
	dbg!(input);
	preceded(wstag(KFUNCTION), funcbody)
		.map(AnonFunctionDefinition)
		.parse(input)
}

pub fn functioncall(input: In) -> IRes<FunctionCall> {
	dbg!(input);
	verify(affix, |affix| {
		matches!(affix.suflist.last().unwrap(), Suffix::Call(_))
	})
	.map(|mut affix| {
		if let Suffix::Call(call) = affix.suflist.pop().unwrap() {
			FunctionCall { affix, call }
		} else {
			// SAFETY: `verify` assured us that the popped element here is an `Call`
			unreachable!()
		}
	})
	.parse(input)
}

pub fn args(input: In) -> IRes<Arguments> {
	dbg!(input);
	use Arguments::*;

	alt((
		paren(opt(explist)).map(ClosedExpressionList),
		tableconstructor.map(TableConstructor),
		literal_string.map(LiteralString),
	))
	.parse(input)
}

pub fn parlist(input: In) -> IRes<ParameterList> {
	dbg!(input);
	use ParameterList::*;

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
