use crate::{
	expression::{Expression, PrefixExpression},
	terminal::Identifier,
};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug)]
pub struct PrefixExpressionIndex {
	pub pexp: Box<PrefixExpression>,
	pub ex: Box<Expression>,
}

impl From<PrefixExpressionIndex> for Variable {
	fn from(value: PrefixExpressionIndex) -> Self {
		Self::PrefixExpressionIndex(value)
	}
}

#[derive(Clone, Debug)]
pub struct PrefixExpressionIdentifier {
	pub pexp: Box<PrefixExpression>,
	pub ident: Identifier,
}

impl From<PrefixExpressionIdentifier> for Variable {
	fn from(value: PrefixExpressionIdentifier) -> Self {
		Self::PrefixExpressionIdentifier(value)
	}
}

#[derive(Clone, Debug)]
pub enum Variable {
	Identifier(Identifier),
	PrefixExpressionIndex(PrefixExpressionIndex),
	PrefixExpressionIdentifier(PrefixExpressionIdentifier),
}

impl From<Variable> for PrefixExpression {
	fn from(value: Variable) -> Self {
		Self::Variable(value)
	}
}
