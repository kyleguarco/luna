use crate::{types::{Identifier, Arguments, ParameterList, Block}, expression::PrefixExpression, statement::Statement};


#[derive(Clone, Debug)]
pub struct FunctionIdentifier {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier>,
}

#[derive(Clone, Debug)]
pub struct CallFunction(PrefixExpression, Arguments);

#[derive(Clone, Debug)]
pub struct CallObjectFunction(PrefixExpression, Identifier, Arguments);

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
