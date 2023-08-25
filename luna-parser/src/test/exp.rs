use luna_ast::{
	expression::{BinaryExpression, Expression, Value},
	operation::BinaryOperation,
	terminal::Numeral,
};

use crate::parse::expression::{exp, value};

#[test]
fn knil() {
	assert_eq!(value("nil"), Ok(("", Value::Nil)));
}

#[test]
fn kfalse() {
	assert_eq!(value("false"), Ok(("", Value::False)));
}

#[test]
fn ktrue() {
	assert_eq!(value("true"), Ok(("", Value::True)));
}

#[test]
fn operators() {
	assert_eq!(
		exp("1+2"),
		Ok((
			"",
			Expression::BinaryExpression(BinaryExpression::AsExpression {
				left: Box::new(Value::Numeral(Numeral::Integer(1))),
				op: BinaryOperation::Add,
				right: Box::new(Expression::BinaryExpression(BinaryExpression::AsValue(
					Box::new(Value::Numeral(Numeral::Integer(2)))
				)))
			})
		))
	);

	assert_eq!(
		exp("3-2"),
		Ok((
			"",
			Expression::BinaryExpression(BinaryExpression::AsExpression {
				left: Box::new(Value::Numeral(Numeral::Integer(3))),
				op: BinaryOperation::Subtract,
				right: Box::new(Expression::BinaryExpression(BinaryExpression::AsValue(
					Box::new(Value::Numeral(Numeral::Integer(2)))
				)))
			})
		))
	);
}
