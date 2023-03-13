///! Compiler types.
///!
///! These are representations of source structure, which are generated
///! from the parser.
use std::boxed::Box;

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
pub struct Block(
	pub Vec<Statement>,
	pub Option<ReturnStatement>,
);

#[derive(Clone, Debug)]
pub enum IfStatement {
	If(Expression, Block),
	ElseIf(Expression, Block),
	Else(Block),
	End,
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
	IfStatement {
		initial: IfStatement,
		belse: IfStatement,
		tree: Vec<IfStatement>,
	},
	ForExpression((
		Identifier,
		(
			Expression,
			Expression,
			Option<Expression>,
		),
		Block,
	)),
	ForList((
		(IdentifierList, ExpressionList),
		Block,
	)),
	FunctionDefinition((FunctionIdentifier, FunctionBody)),
	LocalFunctionDefinition((Identifier, FunctionBody)),
	LocalDefinitionWithAttribute((AttributeNameList, Option<ExpressionList>)),
}

#[derive(Clone, Debug)]
pub struct AttributeName(pub Identifier, pub Attribute);

#[derive(Clone, Debug)]
pub struct Attribute(pub Option<Identifier>);

#[derive(Clone, Debug)]
pub struct AttributeNameList(pub Vec<AttributeName>);

#[derive(Clone, Debug)]
pub struct ReturnStatement(pub Option<ExpressionList>);

#[derive(Clone, Debug)]
pub struct Label(pub Identifier);

impl Into<Statement> for Label {
	fn into(self) -> Statement {
		Statement::Label(self)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionIdentifier {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier>,
}

#[derive(Clone, Debug)]
pub struct VariableList(pub Vec<Variable>);

#[derive(Clone, Debug)]
pub enum Variable {
	Identifier(Identifier),
	PrefixExpressionIndex(Box<PrefixExpression>, Box<Expression>),
	PrefixExpressionIdentifier(Box<PrefixExpression>, Identifier),
}

#[derive(Clone, Debug)]
pub struct IdentifierList(pub Vec<Identifier>);

#[derive(Clone, Debug)]
pub struct ExpressionList(pub Vec<Expression>);

#[derive(Clone, Debug)]
pub enum Expression {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs,
	AnonFunctionDefinition(AnonFunctionDefinition),
	PrefixExpression(Box<PrefixExpression>),
	TableConstructor(TableConstructor),
	InfixOperation(
		Box<Expression>,
		InfixOperation,
		Box<Expression>,
	),
	PrefixOperation(PrefixOperation, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum PrefixExpression {
	Variable(Variable),
	FunctionCall(Box<FunctionCall>),
	ClosedExpression(Expression),
}

#[derive(Clone, Debug)]
pub enum FunctionCall {
	CallFunction(PrefixExpression, Arguments),
	CallObjectFunction(
		PrefixExpression,
		Identifier,
		Arguments,
	),
}

impl Into<Statement> for FunctionCall {
	fn into(self) -> Statement {
		Statement::FunctionCall(self)
	}
}

#[derive(Clone, Debug)]
pub enum Arguments {
	ClosedExpressionList(Option<ExpressionList>),
	TableConstructor(TableConstructor),
	LiteralString(LiteralString),
}

#[derive(Clone, Debug)]
pub struct AnonFunctionDefinition(pub FunctionBody);

#[derive(Clone, Debug)]
pub struct FunctionBody(pub Option<ParameterList>, pub Block);

#[derive(Clone, Debug)]
pub enum ParameterList {
	IdentifierList(IdentifierList),
	IdentifierListWithVarArgs(IdentifierList),
	VarArgs,
}

#[derive(Clone, Debug)]
pub struct TableConstructor(pub Option<FieldList>);

#[derive(Clone, Debug)]
pub struct FieldList(pub Vec<Field>);

#[derive(Clone, Debug)]
pub enum Field {
	BracketField(Expression, Expression),
	IdentifierField(Identifier, Expression),
	Expression(Expression),
}

#[derive(Clone, Debug)]
pub enum InfixOperation {
	Add,
	Subtract,
	Multiply,
	Divide,
	FloorDivide,
	Power,
	Modulo,
	BitwiseAnd,
	BitwiseXor,
	BitwiseOr,
	BitwiseRightShift,
	BitwiseLeftShift,
	Concat,
	LessThan,
	LessEqual,
	GreaterThan,
	GreaterEqual,
	IsEqual,
	IsNotEqual,
	And,
	Or,
}

#[derive(Clone, Debug)]
pub enum PrefixOperation {
	Negate,
	Not,
	Length,
	BitwiseNot,
}
