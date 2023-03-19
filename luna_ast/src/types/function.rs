use super::{Identifier, expression::PrefixExpression, Arguments, statement::Statement, ParameterList, Block};

#[derive(Clone, Debug)]
pub struct FunctionIdentifier {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier>,
}

#[derive(Clone, Debug)]
pub enum FunctionCall {
	CallFunction(PrefixExpression, Arguments),
	CallObjectFunction(PrefixExpression, Identifier, Arguments),
}

impl Into<Statement> for FunctionCall {
	fn into(self) -> Statement {
		Statement::FunctionCall(self)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionBody(pub Option<ParameterList>, pub Block);
