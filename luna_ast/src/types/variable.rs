use super::{
	expression::{Expression, PrefixExpression},
	Identifier,
};

#[derive(Clone, Debug)]
pub struct VariableList(pub Vec<Variable>);

#[derive(Clone, Debug)]
pub enum Variable {
	Identifier(Identifier),
	PrefixExpressionIndex(Box<PrefixExpression>, Box<Expression>),
	PrefixExpressionIdentifier(Box<PrefixExpression>, Identifier),
}
