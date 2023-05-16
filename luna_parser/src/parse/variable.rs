use luna_ast::variable::{PrefixExpressionIndex, PrefixExpressionName, Variable};
use nom::{
	branch::alt,
	character::complete::char as tchar,
	sequence::{pair, separated_pair},
	Parser,
};

use crate::{
	combinator::{bracket, list},
	parse::expression::{exp, prefix_exp},
	terminal::{
		name,
		string::{COMMA, DOT},
	},
	IRes, In,
};

fn prefix_exp_index(input: In) -> IRes<PrefixExpressionIndex> {
	dbg!(input);
	pair(prefix_exp.map(Box::new), bracket(exp).map(Box::new))
		.map(|(pexp, ex)| PrefixExpressionIndex { pexp, ex })
		.parse(input)
}

fn prefix_exp_name(input: In) -> IRes<PrefixExpressionName> {
	dbg!(input);
	separated_pair(prefix_exp.map(Box::from), tchar(DOT), name)
		.map(|(pexp, name)| PrefixExpressionName { pexp, name })
		.parse(input)
}

pub fn var(input: In) -> IRes<Variable> {
	dbg!(input);
	alt((
		name.map(Variable::from),
		prefix_exp_index.map(Variable::from),
		prefix_exp_name.map(Variable::from),
	))
	.parse(input)
}

pub fn var_list(input: In) -> IRes<Vec<Variable>> {
	dbg!(input);
	list(tchar(COMMA), var)(input)
}
