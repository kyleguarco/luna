use std::ops::Range;

use crate::{
	expression::{Expression, ExpressionList},
	function::{FunctionBody, FunctionCall, FunctionIdentifier},
	terminal::{Identifier, IdentifierList},
	variable::VariableList, attribute::AttributeNameList, Block, Label,
};

/// **if** exp **then** block {**elseif** exp **then** block} \[**else** block\] **end**
#[derive(Clone, Debug)]
pub struct IfTree {
	/// The initial condition (if .. then ..)
	pub initial: (Expression, Block),
	/// The tree of statements that follow (executed in order)
	pub elseifs: Vec<(Expression, Block)>,
	/// The last statement to execute of all other conditions are false
	pub otherwise: Option<Block>,
}

/// **for** `ident` **in** `start`, `stop` \[, `step`\] **do** `block` **end**
#[derive(Clone, Debug)]
pub struct ForExpression {
	/// The identifier used in this loop context
	pub ident: Identifier,
	pub range: Range<Expression>,
	pub step: Option<Expression>,
	pub bl: Block,
}

impl From<ForExpression> for Statement {
	fn from(val: ForExpression) -> Self {
		Self::ForExpression(val)
	}
}

#[derive(Clone, Debug)]
pub struct ForList {
	pub ilist: IdentifierList,
	pub elist: ExpressionList,
	pub bl: Block,
}

impl From<ForList> for Statement {
	fn from(value: ForList) -> Self {
		Self::ForList(value)
	}
}

#[derive(Clone, Debug)]
pub struct While {
	pub exp: Expression,
	pub bl: Block,
}

impl From<While> for Statement {
	fn from(value: While) -> Self {
		Self::While(value)
	}
}

#[derive(Clone, Debug)]
pub struct Definition {
	pub varlist: VariableList,
	pub explist: ExpressionList,
}

impl From<Definition> for Statement {
	fn from(value: Definition) -> Self {
		Self::Definition(value)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
	pub ident: FunctionIdentifier,
	pub body: FunctionBody,
}

impl From<FunctionDefinition> for Statement {
	fn from(value: FunctionDefinition) -> Self {
		Self::FunctionDefinition(value)
	}
}

#[derive(Clone, Debug)]
pub struct LocalFunctionDefinition {
	pub ident: Identifier,
	pub body: FunctionBody,
}

impl From<LocalFunctionDefinition> for Statement {
	fn from(value: LocalFunctionDefinition) -> Self {
		Self::LocalFunctionDefinition(value)
	}
}

#[derive(Clone, Debug)]
pub struct LocalDefinitionWithAttribute {
	pub atlist: AttributeNameList,
	pub oelist: Option<ExpressionList>,
}

impl From<LocalDefinitionWithAttribute> for Statement {
	fn from(value: LocalDefinitionWithAttribute) -> Self {
		Self::LocalDefinitionWithAttribute(value)
	}
}

#[derive(Clone, Debug)]
pub enum Statement {
	End,
	Definition(Definition),
	FunctionCall(FunctionCall),
	Label(Label),
	Break,
	Goto(Identifier),
	Do(Box<Block>),
	While(While),
	RepeatUntil(Block, Expression),
	IfTree(IfTree),
	ForExpression(ForExpression),
	ForList(ForList),
	FunctionDefinition(FunctionDefinition),
	LocalFunctionDefinition(LocalFunctionDefinition),
	LocalDefinitionWithAttribute(LocalDefinitionWithAttribute),
}
