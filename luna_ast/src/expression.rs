use crate::{
	function::FunctionCall,
	operation::{InfixOperation, PrefixOperation},
	types::{AnonFunctionDefinition, LiteralString, Numeral, TableConstructor},
	variable::Variable,
};

#[derive(Clone, Debug)]
pub struct ExpressionList(pub Vec<Expression>);

#[derive(Clone, Debug)]
pub struct ReturnStatement(Option<ExpressionList>);

#[derive(Clone, Debug)]
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
	InfixOperation(Box<Expression>, InfixOperation, Box<Expression>),
	PrefixOperation(PrefixOperation, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum PrefixExpression {
	Variable(Variable),
	FunctionCall(Box<FunctionCall>),
	ClosedExpression(Expression),
}
