use crate::{
	function::{FunctionBody, FunctionCall, VarArgs},
	operation::{InfixOperation, UnaryOperation},
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
pub struct AnonFunctionDefinition {
	pub body: FunctionBody,
}

impl From<AnonFunctionDefinition> for Expression {
	fn from(value: AnonFunctionDefinition) -> Self {
		Self::AnonFunctionDefinition(value)
	}
}

/// An infix expression with two parameters.
///
/// Grammar (binop): `exp binop exp`
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
	pub left: Box<Expression>,
	pub op: InfixOperation,
	pub right: Box<Expression>,
}

impl From<BinaryExpression> for Expression {
	fn from(value: BinaryExpression) -> Self {
		Self::BinaryExpression(value)
	}
}

/// A prefix expression with one parameter.
///
/// Grammar (unop): `'-' | <not> | '#' | '~'`
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

/// A group of data that produces a single value.
///
/// Grammar (exp): `'+' | '-' | '*' | '/'
/// | '//' | '^' | '%' | '&' | '~' | '|' | '>>'
/// | '<<' | '..' | '<' | ‘<=' | '>' | ‘>='
/// | '==' | ‘~=' | <and> | <or>`
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs(VarArgs),
	AnonFunctionDefinition(AnonFunctionDefinition),
	PrefixExpression(Box<PrefixExpression>),
	TableConstructor(TableConstructor),
	BinaryExpression(BinaryExpression),
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
