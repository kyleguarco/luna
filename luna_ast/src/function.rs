use crate::{
	affix::{Call, Prefix, Suffix},
	expression::{ExpressionList, Value},
	statement::Statement,
	table::TableConstructor,
	terminal::{LiteralString, Name, NameList},
	Block,
};

#[derive(Clone, Debug, PartialEq)]
pub struct VarArgs;

impl From<VarArgs> for Value {
	fn from(value: VarArgs) -> Self {
		Self::VarArgs(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionName {
	/// Names that refer to a single element or elements of subtables
	pub nlist: Vec<Name>,
	/// Names that refer to table functions that take `self`
	/// as the first parameter.
	pub objname: Option<Name>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionBody {
	pub oplist: Option<ParameterList>,
	pub bl: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
	pub pfix: Prefix,
	pub slist: Vec<Suffix>,
	pub call: Call,
}

impl From<FunctionCall> for Statement {
	fn from(val: FunctionCall) -> Self {
		Statement::FunctionCall(val)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arguments {
	ClosedExpressionList(Option<ExpressionList>),
	TableConstructor(TableConstructor),
	LiteralString(LiteralString),
}

impl From<Option<ExpressionList>> for Arguments {
	fn from(value: Option<ExpressionList>) -> Self {
		Self::ClosedExpressionList(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterList {
	NameList(NameList),
	NameListWithVarArgs(NameList),
	VarArgs(VarArgs),
}

impl From<NameList> for ParameterList {
	fn from(value: NameList) -> Self {
		Self::NameList(value)
	}
}

impl From<(NameList, VarArgs)> for ParameterList {
	fn from(value: (NameList, VarArgs)) -> Self {
		Self::NameListWithVarArgs(value.0)
	}
}

impl From<(VarArgs, NameList)> for ParameterList {
	fn from(value: (VarArgs, NameList)) -> Self {
		Self::NameListWithVarArgs(value.1)
	}
}

impl From<VarArgs> for ParameterList {
	fn from(value: VarArgs) -> Self {
		Self::VarArgs(value)
	}
}
