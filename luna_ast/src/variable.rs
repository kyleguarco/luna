use crate::{
	expression::{Expression, PrefixExpression},
	terminal::Name,
};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExpressionIndex {
	pub pexp: Box<PrefixExpression>,
	pub ex: Box<Expression>,
}

impl From<PrefixExpressionIndex> for Variable {
	fn from(value: PrefixExpressionIndex) -> Self {
		Self::PrefixExpressionIndex(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExpressionName {
	pub pexp: Box<PrefixExpression>,
	pub name: Name,
}

impl From<PrefixExpressionName> for Variable {
	fn from(value: PrefixExpressionName) -> Self {
		Self::PrefixExpressionName(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Name(Name),
	PrefixExpressionIndex(PrefixExpressionIndex),
	PrefixExpressionName(PrefixExpressionName),
}

impl From<Variable> for PrefixExpression {
	fn from(value: Variable) -> Self {
		Self::Variable(value)
	}
}
