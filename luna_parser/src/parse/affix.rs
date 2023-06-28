use luna_ast::affix::{Call, Index, Prefix, Suffix};
use nom::{
	branch::alt,
	combinator::opt,
	sequence::{pair, preceded},
	Parser,
};

use crate::{
	combinator::{braces, bracket, wschar},
	parse::{expression::exp, function::args},
	terminal::{name, string::COLON},
	IRes, In,
};

pub fn prefix(input: In) -> IRes<Prefix> {
	dbg!(input);
	alt((
		braces(exp).map(Prefix::BracedExpression),
		name.map(Prefix::Name),
	))
	.parse(input)
}

pub fn index(input: In) -> IRes<Index> {
	dbg!(input);
	alt((bracket(exp).map(Index::Expression), name.map(Index::Member))).parse(input)
}

pub fn call(input: In) -> IRes<Call> {
	dbg!(input);
	pair(opt(preceded(wschar(COLON), name)), args)
		.map(|(oname, argu)| Call { oname, argu })
		.parse(input)
}

pub fn suffix(input: In) -> IRes<Suffix> {
	dbg!(input);
	alt((call.map(Suffix::Call), index.map(Suffix::Index))).parse(input)
}
