use std::ops::RangeInclusive;

use crate::{
	attribute::AttributeNameList,
	expression::{Expression, ExpressionList},
	function::{FunctionBody, FunctionCall, FunctionName},
	terminal::{Name, NameList},
	variable::VariableList,
	Block,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Label(pub Name);

impl From<Label> for Statement {
	fn from(value: Label) -> Self {
		Self::Label(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfBlock {
	pub cond: Expression,
	pub bl: Block,
}

/// **if** exp **then** block {**elseif** exp **then** block} \[**else** block\] **end**
#[derive(Clone, Debug, PartialEq)]
pub struct IfTree {
	/// The initial condition (if .. then ..)
	pub initial: IfBlock,
	/// The tree of statements that follow (executed in order)
	pub elseifs: Vec<IfBlock>,
	/// The last statement to execute of all other conditions are false
	pub otherwise: Option<Block>,
}

impl From<IfTree> for Statement {
	fn from(value: IfTree) -> Self {
		Self::IfTree(value)
	}
}

/// **for** `name` **in** `start`, `stop` \[, `step`\] **do** `block` **end**
#[derive(Clone, Debug, PartialEq)]
pub struct ForExpression {
	/// The name used in this loop context
	pub name: Name,
	pub range: RangeInclusive<Expression>,
	pub step: Option<Expression>,
	pub bl: Block,
}

impl From<ForExpression> for Statement {
	fn from(val: ForExpression) -> Self {
		Self::ForExpression(val)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForList {
	pub nlist: NameList,
	pub elist: ExpressionList,
	pub bl: Block,
}

impl From<ForList> for Statement {
	fn from(value: ForList) -> Self {
		Self::ForList(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct While {
	pub cond: Expression,
	pub bl: Block,
}

impl From<While> for Statement {
	fn from(value: While) -> Self {
		Self::While(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepeatUntil {
	pub cond: Expression,
	pub bl: Block,
}

impl From<RepeatUntil> for Statement {
	fn from(value: RepeatUntil) -> Self {
		Self::RepeatUntil(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
	pub vlist: VariableList,
	pub elist: ExpressionList,
}

impl From<Definition> for Statement {
	fn from(value: Definition) -> Self {
		Self::Definition(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
	pub fname: FunctionName,
	pub fbody: FunctionBody,
}

impl From<FunctionDefinition> for Statement {
	fn from(value: FunctionDefinition) -> Self {
		Self::FunctionDefinition(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalFunctionDefinition {
	pub name: Name,
	pub fbody: FunctionBody,
}

impl From<LocalFunctionDefinition> for Statement {
	fn from(value: LocalFunctionDefinition) -> Self {
		Self::LocalFunctionDefinition(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalDefinitionWithAttribute {
	pub atlist: AttributeNameList,
	pub oelist: Option<ExpressionList>,
}

impl From<LocalDefinitionWithAttribute> for Statement {
	fn from(value: LocalDefinitionWithAttribute) -> Self {
		Self::LocalDefinitionWithAttribute(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
	End,
	Definition(Definition),
	FunctionCall(FunctionCall),
	Label(Label),
	Break,
	Goto(Name),
	Do(Box<Block>),
	While(While),
	RepeatUntil(RepeatUntil),
	IfTree(IfTree),
	ForExpression(ForExpression),
	ForList(ForList),
	FunctionDefinition(FunctionDefinition),
	LocalFunctionDefinition(LocalFunctionDefinition),
	LocalDefinitionWithAttribute(LocalDefinitionWithAttribute),
}
