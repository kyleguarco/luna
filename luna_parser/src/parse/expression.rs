use luna_ast::expression::{
	AnonFunctionDefinition, BinaryExpression, Expression, UnaryExpression, Value,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{self, opt},
	sequence::{pair, preceded},
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
	function::{funcbody, functioncall, varargs},
	operation::{binop, unop},
	table::tableconstructor,
	variable::var,
};

fn functiondef(input: In) -> IRes<AnonFunctionDefinition> {
	dbg!(input);
	preceded(tag(KFUNCTION), funcbody)
		.map(AnonFunctionDefinition)
		.parse(input)
}

fn binary_op(input: In) -> IRes<BinaryExpression> {
	use BinaryExpression::*;

	dbg!(input);
	value
		.map(Box::new)
		.and(opt(pair(binop, exp.map(Box::new))))
		.map(|(left, oright)| match oright {
			Some((op, right)) => AsExpression { left, op, right },
			None => AsValue(left),
		})
		.parse(input)
}

fn unary_op(input: In) -> IRes<UnaryExpression> {
	dbg!(input);
	pair(unop, exp.map(Box::new))
		.map(|(op, ex)| UnaryExpression { op, ex })
		.parse(input)
}

pub fn value(input: In) -> IRes<Value> {
	use Value::*;

	dbg!(input);
	alt((
		combinator::value(Nil, tag(KNIL)),
		combinator::value(False, tag(KFALSE)),
		combinator::value(True, tag(KTRUE)),
		numeral.map(Numeral),
		literal_string.map(LiteralString),
		varargs.map(VarArgs),
		functiondef.map(AnonFunctionDefinition),
		var.map(Variable),
		functioncall.map(FunctionCall),
		braces(exp).map(BracedExpression),
		tableconstructor.map(TableConstructor),
	))
	.parse(input)
}

pub fn exp(input: In) -> IRes<Expression> {
	dbg!(input);
	alt((
		binary_op.map(Expression::BinaryExpression),
		unary_op.map(Expression::UnaryExpression),
	))
	.parse(input)
}

pub fn explist(input: In) -> IRes<Vec<Expression>> {
	dbg!(input);
	list(tchar(COMMA), exp)(input)
}
