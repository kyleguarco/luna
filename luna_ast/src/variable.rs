use crate::{
	expression::{Expression, PrefixExpression},
	terminal::Identifier,
};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug)]
pub enum Variable {
	Identifier(Identifier),
	PrefixExpressionIndex(Box<PrefixExpression>, Box<Expression>),
	PrefixExpressionIdentifier(Box<PrefixExpression>, Identifier),
}
