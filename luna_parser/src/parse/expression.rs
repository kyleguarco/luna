use luna_ast::expression::{
	AnonFunctionDefinition, Expression, InfixExpression, PrefixExpression, UnaryExpression,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::value,
	sequence::{pair, preceded, tuple},
	Parser,
};

use crate::{
	combinator::{braces, list},
	terminal::{
		keyword::{KFALSE, KFUNCTION, KNIL, KTRUE},
		literal_string, numeral,
		string::COMMA,
	},
	IRes, In,
};

use super::{
	function::{func_body, func_call, var_args},
	operation::{infix_op, unary_op},
	table::table_cons,
	variable::var,
};

fn anon_func_def(input: In) -> IRes<AnonFunctionDefinition> {
	dbg!(input);
	preceded(tag(KFUNCTION), func_body)
		.map(|body| AnonFunctionDefinition { body })
		.parse(input)
}

fn infix_exp(input: In) -> IRes<InfixExpression> {
	dbg!(input);
	tuple((exp.map(Box::new), infix_op, exp.map(Box::new)))
		.map(|(left, op, right)| InfixExpression { left, op, right })
		.parse(input)
}

fn unary_exp(input: In) -> IRes<UnaryExpression> {
	dbg!(input);
	pair(unary_op, exp.map(Box::new))
		.map(|(op, ex)| UnaryExpression { op, ex })
		.parse(input)
}

pub fn exp(input: In) -> IRes<Expression> {
	dbg!(input);
	alt((
		value(Expression::Nil, tag(KNIL)),
		value(Expression::False, tag(KFALSE)),
		value(Expression::True, tag(KTRUE)),
		numeral.map(Expression::from),
		literal_string.map(Expression::from),
		var_args.map(Expression::from),
		anon_func_def.map(Expression::from),
		prefix_exp.map(Expression::from),
		table_cons.map(Expression::from),
		infix_exp.map(Expression::from),
		unary_exp.map(Expression::from),
	))
	.parse(input)
}

pub fn prefix_exp(input: In) -> IRes<PrefixExpression> {
	dbg!(input);
	alt((
		var.map(PrefixExpression::from),
		func_call.map(PrefixExpression::from),
		braces(exp).map(PrefixExpression::from),
	))
	.parse(input)
}

pub fn exp_list(input: In) -> IRes<Vec<Expression>> {
	dbg!(input);
	list(tchar(COMMA), exp)(input)
}
