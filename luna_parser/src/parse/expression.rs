use luna_ast::expression::{
	AnonFunctionDefinition, BinaryExpression, Expression, PrefixExpression, UnaryExpression,
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
	function::{funcbody, functioncall, var_args},
	operation::{binop, unop},
	table::tableconstructor,
	variable::var,
};

fn functiondef(input: In) -> IRes<AnonFunctionDefinition> {
	dbg!(input);
	preceded(tag(KFUNCTION), funcbody)
		.map(|body| AnonFunctionDefinition { body })
		.parse(input)
}

fn binary_op(input: In) -> IRes<BinaryExpression> {
	dbg!(input);
	tuple((exp.map(Box::new), binop, exp.map(Box::new)))
		.map(|(left, op, right)| BinaryExpression { left, op, right })
		.parse(input)
}

fn unary_op(input: In) -> IRes<UnaryExpression> {
	dbg!(input);
	pair(unop, exp.map(Box::new))
		.map(|(op, ex)| UnaryExpression { op, ex })
		.parse(input)
}

pub fn exp(input: In) -> IRes<Expression> {
	use Expression::*;

	dbg!(input);
	alt((
		value(Expression::Nil, tag(KNIL)),
		value(Expression::False, tag(KFALSE)),
		value(Expression::True, tag(KTRUE)),
		numeral.map(Numeral),
		literal_string.map(LiteralString),
		var_args.map(VarArgs),
		functiondef.map(AnonFunctionDefinition),
		prefixexp.map(Box::new).map(PrefixExpression),
		tableconstructor.map(TableConstructor),
		binary_op.map(BinaryExpression),
		unary_op.map(UnaryExpression),
	))
	.parse(input)
}

pub fn prefixexp(input: In) -> IRes<PrefixExpression> {
	dbg!(input);
	alt((
		var.map(PrefixExpression::from),
		functioncall.map(PrefixExpression::from),
		braces(exp).map(PrefixExpression::from),
	))
	.parse(input)
}

pub fn explist(input: In) -> IRes<Vec<Expression>> {
	dbg!(input);
	list(tchar(COMMA), exp)(input)
}
