use super::{
	expression::{Expression, ExpressionList},
	function::{FunctionBody, FunctionCall, FunctionIdentifier},
	variable::VariableList,
	AttributeNameList, Block, Identifier, IdentifierList, Label,
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
	pub start: Expression,
	pub stop: Expression,
	pub step: Option<Expression>,
	pub bl: Block,
}

impl Into<Statement> for ForExpression {
	fn into(self) -> Statement {
		Statement::ForExpression(self)
	}
}

#[derive(Clone, Debug)]
pub enum Statement {
	End,
	Definition((VariableList, ExpressionList)),
	FunctionCall(FunctionCall),
	Label(Label),
	Break,
	Goto(Identifier),
	Do(Box<Block>),
	While((Expression, Block)),
	RepeatUntil(Block, Expression),
	IfTree(IfTree),
	ForExpression(ForExpression),
	ForList(((IdentifierList, ExpressionList), Block)),
	FunctionDefinition((FunctionIdentifier, FunctionBody)),
	LocalFunctionDefinition((Identifier, FunctionBody)),
	LocalDefinitionWithAttribute((AttributeNameList, Option<ExpressionList>)),
}
