use crate::{
	function::{FunctionBody, FunctionCall},
	operation::{InfixOperation, UnaryOperation},
	table::{Field, TableConstructor},
	terminal::{LiteralString, Numeral},
	variable::Variable,
};

pub type ExpressionList = Vec<Expression>;

#[derive(Clone, Debug, PartialEq)]
pub struct AnonFunctionDefinition {
	pub body: FunctionBody,
}

impl From<AnonFunctionDefinition> for Expression {
	fn from(value: AnonFunctionDefinition) -> Self {
		Self::AnonFunctionDefinition(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixExpression {
	pub left: Box<Expression>,
	pub op: InfixOperation,
	pub right: Box<Expression>,
}

impl From<InfixExpression> for Expression {
	fn from(value: InfixExpression) -> Self {
		Self::InfixExpression(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
	pub op: UnaryOperation,
	pub ex: Box<Expression>,
}

impl From<UnaryExpression> for Expression {
	fn from(value: UnaryExpression) -> Self {
		Self::UnaryExpression(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs,
	AnonFunctionDefinition(AnonFunctionDefinition),
	PrefixExpression(Box<PrefixExpression>),
	TableConstructor(TableConstructor),
	InfixExpression(InfixExpression),
	UnaryExpression(UnaryExpression),
}

impl From<Expression> for Field {
	fn from(value: Expression) -> Self {
		Self::Expression(value)
	}
}

impl From<Expression> for PrefixExpression {
	fn from(value: Expression) -> Self {
		Self::ClosedExpression(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefixExpression {
	Variable(Variable),
	FunctionCall(Box<FunctionCall>),
	ClosedExpression(Expression),
}

impl From<PrefixExpression> for Expression {
	fn from(value: PrefixExpression) -> Self {
		Self::PrefixExpression(Box::new(value))
	}
}
