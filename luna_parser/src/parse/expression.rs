use luna_ast::expression::{BinaryExpression, Expression, UnaryExpression, Value};
use nom::{
	branch::alt,
	combinator::{self, opt},
	sequence::pair,
	Parser,
};

use crate::{
	combinator::{list, paren, wschar, wstag},
	parse::function::functiondef,
	terminal::{
		keyword::{KFALSE, KNIL, KTRUE},
		literal_string, numeral,
		string::COMMA,
	},
	IRes, In,
};

use super::{
	function::{functioncall, varargs},
	operation::{binop, unop},
	table::tableconstructor,
	variable::var,
};

fn binary_op(input: In) -> IRes<BinaryExpression> {
	dbg!(input);
	use BinaryExpression::*;

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
	dbg!(input);
	use Value::*;

	alt((
		combinator::value(Nil, wstag(KNIL)),
		combinator::value(False, wstag(KFALSE)),
		combinator::value(True, wstag(KTRUE)),
		numeral.map(Numeral),
		literal_string.map(LiteralString),
		varargs.map(VarArgs),
		functiondef.map(AnonFunctionDefinition),
		var.map(Variable),
		functioncall.map(FunctionCall),
		paren(exp).map(ParenExpression),
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
	list(wschar(COMMA), exp)(input)
}
