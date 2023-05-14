use luna_ast::variable::{PrefixExpressionName, PrefixExpressionIndex, Variable};
use nom::{
	branch::alt,
	character::complete::char as tchar,
	sequence::{pair, separated_pair},
	Parser,
};

use crate::{
	combinator::bracket,
	parse::expression::{exp, prefix_exp},
	terminal::{name, string::DOT},
	IRes, In,
};

pub fn prefix_exp_index(input: In) -> IRes<PrefixExpressionIndex> {
	pair(prefix_exp.map(Box::new), bracket(exp).map(Box::new))
		.map(|(pexp, ex)| PrefixExpressionIndex { pexp, ex })
		.parse(input)
}

pub fn prefix_exp_name(input: In) -> IRes<PrefixExpressionName> {
	separated_pair(prefix_exp.map(Box::from), tchar(DOT), name)
		.map(|(pexp, name)| PrefixExpressionName { pexp, name })
		.parse(input)
}

pub fn var(input: In) -> IRes<Variable> {
	alt((
		name.map(Variable::from),
		prefix_exp_index.map(Variable::from),
		prefix_exp_name.map(Variable::from),
	))
	.parse(input)
}
