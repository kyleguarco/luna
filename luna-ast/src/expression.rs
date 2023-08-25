use crate::{
	function::{FunctionBody, FunctionCall, VarArgs},
	operation::{BinaryOperation, UnaryOperation},
	table::{Field, TableConstructor},
	terminal::{LiteralString, Numeral},
	variable::Variable,
};

/// Grammar: `exp {',' exp}`
pub type ExpressionList = Vec<Expression>;

/// Defintion for an anonymous Lua function, usually used like this:
/// ```lua
/// local anon = function() end
/// ```
/// Grammar (functiondef): `<function> funcbody`
#[derive(Clone, Debug, PartialEq)]
pub struct AnonFunctionDefinition(pub FunctionBody);

impl From<AnonFunctionDefinition> for Value {
	fn from(value: AnonFunctionDefinition) -> Self {
		Self::AnonFunctionDefinition(value)
	}
}

/// An infix expression with two parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryExpression {
	AsValue(Box<Value>),
	AsExpression {
		left: Box<Value>,
		op: BinaryOperation,
		right: Box<Expression>,
	},
}

impl From<BinaryExpression> for Expression {
	fn from(value: BinaryExpression) -> Self {
		Self::BinaryExpression(value)
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
pub enum Value {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs(VarArgs),
	AnonFunctionDefinition(AnonFunctionDefinition),
	Variable(Variable),
	FunctionCall(FunctionCall),
	ParenExpression(Expression),
	TableConstructor(TableConstructor),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	BinaryExpression(BinaryExpression),
	UnaryExpression(UnaryExpression),
}

impl From<Expression> for Field {
	fn from(value: Expression) -> Self {
		Self::Expression(value)
	}
}

impl From<Expression> for Value {
	fn from(value: Expression) -> Self {
		Self::ParenExpression(value)
	}
}
