use crate::{
	expression::{Expression, ExpressionList, PrefixExpression},
	statement::Statement,
	table::TableConstructor,
	terminal::{LiteralString, Name, NameList},
	Block,
};

#[derive(Clone, Debug)]
pub struct VarArgs;

impl From<VarArgs> for Expression {
	fn from(_: VarArgs) -> Self {
		Self::VarArgs
	}
}

#[derive(Clone, Debug)]
pub struct FunctionName {
	/// Names that refer to a single element or elements of subtables
	pub nlist: Vec<Name>,
	/// Names that refer to table functions that take `self`
	/// as the first parameter.
	pub objname: Option<Name>,
}

#[derive(Clone, Debug)]
pub struct AsFunction {
	pub pexp: PrefixExpression,
	pub argu: Arguments,
}

impl From<AsFunction> for FunctionCall {
	fn from(value: AsFunction) -> Self {
		Self::AsFunction(value)
	}
}

#[derive(Clone, Debug)]
pub struct AsMethod {
	pub pexp: PrefixExpression,
	pub name: Name,
	pub argu: Arguments,
}

impl From<AsMethod> for FunctionCall {
	fn from(value: AsMethod) -> Self {
		Self::AsMethod(value)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionBody {
	pub oplist: Option<ParameterList>,
	pub bl: Block,
}

#[derive(Clone, Debug)]
pub enum FunctionCall {
	/// A function without a `self` parameter.
	AsFunction(AsFunction),
	/// A function with a `self` parameter.
	AsMethod(AsMethod),
}

impl From<FunctionCall> for Statement {
	fn from(val: FunctionCall) -> Self {
		Statement::FunctionCall(val)
	}
}

impl From<FunctionCall> for PrefixExpression {
	fn from(value: FunctionCall) -> Self {
		Self::FunctionCall(Box::new(value))
	}
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum ParameterList {
	NameList(NameList),
	NameListWithVarArgs(NameList),
	VarArgs,
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
	fn from(_: VarArgs) -> Self {
		Self::VarArgs
	}
}
