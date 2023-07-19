use luna_ast::affix::{Affix, Call, Index, Prefix, Suffix};
use nom::{
	branch::alt,
	combinator::opt,
	multi::many1,
	sequence::{pair, preceded},
	Parser,
};

use crate::{
	combinator::{bracket, paren, wschar},
	parse::{expression::exp, function::args},
	terminal::{
		name,
		string::{COLON, DOT},
	},
	IRes, In,
};

pub fn prefix(input: In) -> IRes<Prefix> {
	dbg!(input);
	alt((
		paren(exp).map(Prefix::ParenExpression),
		name.map(Prefix::Name),
	))
	.parse(input)
}

pub fn index(input: In) -> IRes<Index> {
	dbg!(input);
	alt((
		bracket(exp).map(Index::Expression),
		preceded(wschar(DOT), name).map(Index::Member),
	))
	.parse(input)
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

pub fn affix(input: In) -> IRes<Affix> {
	dbg!(input);
	prefix
		.and(many1(suffix))
		.map(|(pfix, suflist)| Affix { pfix, suflist })
		.parse(input)
}
