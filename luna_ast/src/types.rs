///! Compiler types.
///!
///! These are representations of source pub structure, which are generated
///! from the parser.
use std::boxed::Box;

#[derive(Clone, Debug)]
pub struct Identifier<'ident>(pub &'ident str);

#[derive(Clone)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

#[derive(Clone)]
pub struct LiteralString;

pub struct Chunk<'ident>(pub Block<'ident>);

pub struct Block<'ident>(
	pub Vec<Statement<'ident>>,
	pub Option<ReturnStatement<'ident>>,
);

pub enum IfStatement<'ident> {
	If(Expression<'ident>, Block<'ident>),
	ElseIf(Expression<'ident>, Block<'ident>),
	Else(Block<'ident>),
	End,
}

pub enum Statement<'ident> {
	End,
	Definition(VariableList<'ident>, ExpressionList<'ident>),
	FunctionCall(FunctionCall<'ident>),
	Label(Label<'ident>),
	Break,
	Goto(Identifier<'ident>),
	Do(Box<Block<'ident>>),
	While(Expression<'ident>, Block<'ident>),
	RepeatUntil(Block<'ident>, Expression<'ident>),
	IfStatement {
		initial: IfStatement<'ident>,
		belse: IfStatement<'ident>,
		tree: Vec<IfStatement<'ident>>,
	},
	ForExpression(
		Identifier<'ident>,
		(
			Expression<'ident>,
			Expression<'ident>,
			Option<Expression<'ident>>,
		),
		Block<'ident>,
	),
	ForList(
		IdentifierList<'ident>,
		ExpressionList<'ident>,
		Block<'ident>,
	),
	FunctionDefinition(FunctionIdentifier<'ident>, FunctionBody),
	LocalFunctionDefinition(Identifier<'ident>, FunctionBody),
	LocalDefinitionWithAttribute(AttributeNameList<'ident>, Option<ExpressionList<'ident>>),
}

pub struct AttributeName<'ident>(pub Identifier<'ident>, pub Attribute<'ident>);

pub struct Attribute<'ident>(pub Option<Identifier<'ident>>);

pub struct AttributeNameList<'ident>(pub Vec<AttributeName<'ident>>);

pub struct ReturnStatement<'ident>(pub Option<ExpressionList<'ident>>);

pub struct Label<'ident>(pub Identifier<'ident>);

impl<'ident> Into<Statement<'ident>> for Label<'ident> {
	fn into(self) -> Statement<'ident> {
		Statement::Label(self)
	}
}

pub struct FunctionIdentifier<'ident> {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier<'ident>>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier<'ident>>,
}

pub struct VariableList<'ident>(pub Vec<Variable<'ident>>);

#[derive(Clone)]
pub enum Variable<'ident> {
	Identifier(Identifier<'ident>),
	PrefixExpressionIndex(Box<PrefixExpression<'ident>>, Box<Expression<'ident>>),
	PrefixExpressionIdentifier(Box<PrefixExpression<'ident>>, Identifier<'ident>),
}

pub struct IdentifierList<'ident>(pub Vec<Identifier<'ident>>);

pub struct ExpressionList<'ident>(pub Vec<Expression<'ident>>);

#[derive(Clone)]
pub enum Expression<'ident> {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs,
	AnonFunctionDefinition(AnonFunctionDefinition),
	PrefixExpression(Box<PrefixExpression<'ident>>),
	TableConstructor(TableConstructor),
	InfixOperation(
		Box<Expression<'ident>>,
		InfixOperation,
		Box<Expression<'ident>>,
	),
	PrefixOperation(PrefixOperation, Box<Expression<'ident>>),
}

#[derive(Clone)]
pub enum PrefixExpression<'ident> {
	Variable(Variable<'ident>),
	FunctionCall(Box<FunctionCall<'ident>>),
	ClosedExpression(Expression<'ident>),
}

#[derive(Clone)]
pub enum FunctionCall<'ident> {
	CallFunction(PrefixExpression<'ident>, Arguments),
	CallObjectFunction(PrefixExpression<'ident>, Identifier<'ident>, Arguments),
}

impl<'ident> Into<Statement<'ident>> for FunctionCall<'ident> {
	fn into(self) -> Statement<'ident> {
		Statement::FunctionCall(self)
	}
}

#[derive(Clone)]
pub enum Arguments {
	ClosedExpressionList,
	TableConstructor,
	LiteralString,
}

#[derive(Clone)]
pub struct AnonFunctionDefinition(FunctionBody);

#[derive(Clone)]
pub struct FunctionBody;

pub enum ParameterList {
	IdentifierList,
	IdentifierListTripleDots,
	TripleDots,
}

#[derive(Clone)]
pub struct TableConstructor;

pub struct FieldList;

pub enum Field {
	BracketExpression,
	IdentifierExpression,
	Expression,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum PrefixOperation {
	Negate,
	Not,
	Length,
	BitwiseNot,
}
