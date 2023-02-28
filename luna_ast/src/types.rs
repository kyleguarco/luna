///! Compiler types.
///!
///! These are representations of source pub structure, which are generated
///! from the parser.
use std::boxed::Box;

#[derive(Debug)]
pub struct Identifier<'a>(pub &'a str);

#[derive(Clone)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

#[derive(Clone)]
pub struct LiteralString;

pub struct Chunk<'a>(pub Block<'a>);

pub struct Block<'a>(pub Vec<Statement<'a>>, pub Option<ReturnStatement>);

pub enum IfStatement<'a> {
	If(Expression, Block<'a>),
	ElseIf(Expression, Block<'a>),
	Else(Block<'a>),
	End,
}

pub enum Statement<'a> {
	End,
	Definition(VariableList<'a>, ExpressionList),
	FunctionCall(FunctionCall),
	Label(Label<'a>),
	Break,
	Goto(Identifier<'a>),
	Do(Box<Block<'a>>),
	While(Expression, Block<'a>),
	RepeatUntil(Block<'a>, Expression),
	IfStatement {
		initial: IfStatement<'a>,
		belse: IfStatement<'a>,
		tree: Vec<IfStatement<'a>>,
	},
	ForExpression(
		Identifier<'a>,
		(Expression, Expression, Option<Expression>),
		Block<'a>,
	),
	ForList(IdentifierList<'a>, ExpressionList, Block<'a>),
	FunctionDefinition(FunctionIdentifier<'a>, FunctionBody),
	LocalFunctionDefinition(Identifier<'a>, FunctionBody),
	LocalDefinitionWithAttribute(AttributeNameList<'a>, Option<ExpressionList>),
}

pub struct AttributeName<'a>(pub Identifier<'a>, pub Attribute<'a>);

pub struct Attribute<'a>(pub Option<Identifier<'a>>);

pub struct AttributeNameList<'a>(pub Vec<AttributeName<'a>>);

pub struct ReturnStatement(pub Option<ExpressionList>);

pub struct Label<'a>(pub Identifier<'a>);

impl<'a> Into<Statement<'a>> for Label<'a> {
	fn into(self) -> Statement<'a> {
		Statement::Label(self)
	}
}

pub struct FunctionIdentifier<'a> {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier<'a>>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier<'a>>,
}

pub struct VariableList<'a>(pub Vec<Variable<'a>>);

pub enum Variable<'a> {
	Identifier(Identifier<'a>),
	PrefixExpressionIndex(PrefixExpression, Expression),
	PrefixExpressionIdentifier(PrefixExpression, Identifier<'a>),
}

pub struct IdentifierList<'a>(pub Vec<Identifier<'a>>);

pub struct ExpressionList(pub Vec<Expression>);

#[derive(Clone)]
pub enum Expression {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs,
	AnonFunctionDefinition(AnonFunctionDefinition),
	PrefixExpression(PrefixExpression),
	TableConstructor(TableConstructor),
	InfixOperation(Box<Expression>, InfixOperation, Box<Expression>),
	PrefixOperation(PrefixOperation, Box<Expression>),
}

#[derive(Clone)]
pub enum PrefixExpression {
	Variable,
	FunctionCall,
	ClosedExpression,
}

pub enum FunctionCall {
	CallFunction,
	CallObjectFunction,
}

impl<'a> Into<Statement<'a>> for FunctionCall {
	fn into(self) -> Statement<'a> {
		Statement::FunctionCall(self)
	}
}

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
