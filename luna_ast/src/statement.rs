use std::ops::Range;

use crate::{
	expression::{Expression, ExpressionList},
	function::{FunctionBody, FunctionCall, FunctionIdentifier},
	types::{AttributeNameList, Block, Identifier, IdentifierList, Label},
	variable::VariableList,
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
		Statement::ForExpression(val)
	}
}

#[derive(Clone, Debug)]
pub struct ForList {
	pub ilist: IdentifierList,
	pub elist: ExpressionList,
	pub bl: Block,
}

#[derive(Clone, Debug)]
pub struct While(Expression, Block);

#[derive(Clone, Debug)]
pub struct Definition(VariableList, ExpressionList);

#[derive(Clone, Debug)]
pub struct FunctionDefinition(FunctionIdentifier, FunctionBody);

#[derive(Clone, Debug)]
pub struct LocalFunctionDefinition(Identifier, FunctionBody);

#[derive(Clone, Debug)]
pub struct LocalDefinitionWithAttribute(AttributeNameList, Option<ExpressionList>);

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
