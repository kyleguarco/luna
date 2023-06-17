use luna_ast::variable::{PrefixExpressionIndex, PrefixExpressionName, Variable};
use nom::{
	branch::alt,
	character::complete::char as tchar,
	sequence::{pair, separated_pair},
	Parser,
};

use crate::{
	combinator::{bracket, list},
	parse::expression::{exp, prefixexp},
	terminal::{
		name,
		string::{COMMA, DOT},
	},
	IRes, In,
};

fn prefix_exp_index(input: In) -> IRes<PrefixExpressionIndex> {
	dbg!(input);
	pair(prefixexp.map(Box::new), bracket(exp).map(Box::new))
		.map(|(pexp, ex)| PrefixExpressionIndex { pexp, ex })
		.parse(input)
}

fn prefix_exp_name(input: In) -> IRes<PrefixExpressionName> {
	dbg!(input);
	separated_pair(prefixexp.map(Box::from), tchar(DOT), name)
		.map(|(pexp, name)| PrefixExpressionName { pexp, name })
		.parse(input)
}

pub fn var(input: In) -> IRes<Variable> {
	use Variable::*;

	dbg!(input);
	alt((
		name.map(Name),
		prefix_exp_index.map(PrefixExpressionIndex),
		prefix_exp_name.map(PrefixExpressionName),
	))
	.parse(input)
}

pub fn varlist(input: In) -> IRes<Vec<Variable>> {
	dbg!(input);
	list(tchar(COMMA), var)(input)
}
