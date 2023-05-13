use crate::{
	expression::{ExpressionList, PrefixExpression},
	statement::Statement,
	table::TableConstructor,
	terminal::{Identifier, IdentifierList, LiteralString}, Block,
};

#[derive(Clone, Debug)]
pub struct FunctionIdentifier {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier>,
}

#[derive(Clone, Debug)]
pub struct CallFunction {
	pub prefix: PrefixExpression,
	pub args: Arguments,
}

#[derive(Clone, Debug)]
pub struct CallObjectFunction {
	pub prefix: PrefixExpression,
	pub ident: Identifier,
	pub args: Arguments,
}

#[derive(Clone, Debug)]
pub enum FunctionCall {
	CallFunction(CallFunction),
	CallObjectFunction(CallObjectFunction),
}

impl From<FunctionCall> for Statement {
	fn from(val: FunctionCall) -> Self {
		Statement::FunctionCall(val)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionBody {
	pub plist: Option<ParameterList>,
	pub block: Block,
}

#[derive(Clone, Debug)]
pub enum Arguments {
	ClosedExpressionList(Option<ExpressionList>),
	TableConstructor(TableConstructor),
	LiteralString(LiteralString),
}

#[derive(Clone, Debug)]
pub struct AnonFunctionDefinition {
	pub body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum ParameterList {
	IdentifierList(IdentifierList),
	IdentifierListWithVarArgs(IdentifierList),
	VarArgs,
}
