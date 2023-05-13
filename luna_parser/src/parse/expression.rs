use luna_ast::expression::{
	AnonFunctionDefinition, Expression, InfixExpression, PrefixExpression, UnaryExpression,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::value,
	sequence::{delimited, pair, preceded, tuple},
	Parser,
};

use crate::{
	combinator::braces,
	terminal::{
		keyword::{KFALSE, KFUNCTION, KNIL, KTRUE},
		literal_string, numeral,
		string::TRIPLEDOT,
	},
	IRes, In,
};

use super::{
	function::{func_body, func_call},
	operation::{infix_op, unary_op},
	table::table_cons,
	variable::var,
};

pub fn anon_func_def(input: In) -> IRes<AnonFunctionDefinition> {
	preceded(tag(KFUNCTION), func_body)
		.map(|body| AnonFunctionDefinition { body })
		.parse(input)
}

pub fn infix_exp(input: In) -> IRes<InfixExpression> {
	tuple((exp.map(Box::new), infix_op, exp.map(Box::new)))
		.map(|(left, op, right)| InfixExpression { left, op, right })
		.parse(input)
}

pub fn unary_exp(input: In) -> IRes<UnaryExpression> {
	pair(unary_op, exp.map(Box::new))
		.map(|(op, ex)| UnaryExpression { op, ex })
		.parse(input)
}

pub fn exp(input: In) -> IRes<Expression> {
	alt((
		value(Expression::Nil, tag(KNIL)),
		value(Expression::False, tag(KFALSE)),
		value(Expression::True, tag(KTRUE)),
		numeral.map(Expression::from),
		literal_string.map(Expression::from),
		value(Expression::VarArgs, tag(TRIPLEDOT)),
		anon_func_def.map(Expression::from),
		prefix_exp.map(Expression::from),
		table_cons.map(Expression::from),
		infix_exp.map(Expression::from),
		unary_exp.map(Expression::from),
	))
	.parse(input)
}

pub fn prefix_exp(input: In) -> IRes<PrefixExpression> {
	alt((
		var.map(PrefixExpression::from),
		func_call.map(PrefixExpression::from),
		braces(exp).map(PrefixExpression::from),
	))
	.parse(input)
}
