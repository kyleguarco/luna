//! # Compiler types.
//!
//! These are representations of source structure, which are generated
//! from the parser.

use crate::{
	attribute::AttributeName,
	expression::{Expression, ExpressionList, ReturnStatement},
	function::FunctionBody,
	statement::Statement,
};

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

#[derive(Clone, Debug)]
pub struct LiteralString(pub String);

#[derive(Clone, Debug)]
pub struct Chunk(pub Block);

#[derive(Clone, Debug)]
pub struct Block {
	/// The statements within this block.
	pub stlist: Vec<Statement>,
	/// The return statement, if any. Void if `None`.
	pub oret: Option<ReturnStatement>,
}

impl Block {
	pub fn from_parser(arg: (Vec<Statement>, Option<ReturnStatement>)) -> Self {
		Self { stlist: arg.0, oret: arg.1 }
	}
}

#[derive(Clone, Debug)]
pub struct AttributeNameList(pub Vec<AttributeName>);

#[derive(Clone, Debug)]
pub struct Label(pub Identifier);

impl From<Label> for Statement {
	fn from(val: Label) -> Self {
		Statement::Label(val)
	}
}

pub type IdentifierList = Vec<Identifier>;

#[derive(Clone, Debug)]
pub enum Arguments {
	ClosedExpressionList(Option<ExpressionList>),
	TableConstructor(TableConstructor),
	LiteralString(LiteralString),
}

#[derive(Clone, Debug)]
pub struct AnonFunctionDefinition(pub FunctionBody);

#[derive(Clone, Debug)]
pub enum ParameterList {
	IdentifierList(IdentifierList),
	IdentifierListWithVarArgs(IdentifierList),
	VarArgs,
}

#[derive(Clone, Debug)]
pub struct TableConstructor(pub Option<FieldList>);

pub type FieldList = Vec<Field>;

#[derive(Clone, Debug)]
pub enum Field {
	BracketField((Expression, Expression)),
	IdentifierField((Identifier, Expression)),
	Expression(Expression),
}
