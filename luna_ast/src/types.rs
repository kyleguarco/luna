///! Compiler types.
///!
///! These are representations of source structure, which are generated
///! from the parser.
use std::boxed::Box;

#[derive(Clone, Debug)]
pub struct Identifier<'ident>(pub &'ident str);

#[derive(Clone, Debug)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

#[derive(Clone, Debug)]
pub struct LiteralString;

#[derive(Clone, Debug)]
pub struct Chunk<'ident>(pub Block<'ident>);

#[derive(Clone, Debug)]
pub struct Block<'ident>(
	pub Vec<Statement<'ident>>,
	pub Option<ReturnStatement<'ident>>,
);

#[derive(Clone, Debug)]
pub enum IfStatement<'ident> {
	If(Expression<'ident>, Block<'ident>),
	ElseIf(Expression<'ident>, Block<'ident>),
	Else(Block<'ident>),
	End,
}

#[derive(Clone, Debug)]
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
	FunctionDefinition(FunctionIdentifier<'ident>, FunctionBody<'ident>),
	LocalFunctionDefinition(Identifier<'ident>, FunctionBody<'ident>),
	LocalDefinitionWithAttribute(AttributeNameList<'ident>, Option<ExpressionList<'ident>>),
}

#[derive(Clone, Debug)]
pub struct AttributeName<'ident>(pub Identifier<'ident>, pub Attribute<'ident>);

#[derive(Clone, Debug)]
pub struct Attribute<'ident>(pub Option<Identifier<'ident>>);

#[derive(Clone, Debug)]
pub struct AttributeNameList<'ident>(pub Vec<AttributeName<'ident>>);

#[derive(Clone, Debug)]
pub struct ReturnStatement<'ident>(pub Option<ExpressionList<'ident>>);

#[derive(Clone, Debug)]
pub struct Label<'ident>(pub Identifier<'ident>);

impl<'ident> Into<Statement<'ident>> for Label<'ident> {
	fn into(self) -> Statement<'ident> {
		Statement::Label(self)
	}
}

#[derive(Clone, Debug)]
pub struct FunctionIdentifier<'ident> {
	/// Identifiers that refer to a single element or elements of subtables
	pub ilist: Vec<Identifier<'ident>>,
	/// Identifiers that refer to table functions that take `self`
	/// as the first parameter.
	pub objident: Option<Identifier<'ident>>,
}

#[derive(Clone, Debug)]
pub struct VariableList<'ident>(pub Vec<Variable<'ident>>);

#[derive(Clone, Debug)]
pub enum Variable<'ident> {
	Identifier(Identifier<'ident>),
	PrefixExpressionIndex(Box<PrefixExpression<'ident>>, Box<Expression<'ident>>),
	PrefixExpressionIdentifier(Box<PrefixExpression<'ident>>, Identifier<'ident>),
}

#[derive(Clone, Debug)]
pub struct IdentifierList<'ident>(pub Vec<Identifier<'ident>>);

#[derive(Clone, Debug)]
pub struct ExpressionList<'ident>(pub Vec<Expression<'ident>>);

#[derive(Clone, Debug)]
pub enum Expression<'ident> {
	Nil,
	False,
	True,
	Numeral(Numeral),
	LiteralString(LiteralString),
	VarArgs,
	AnonFunctionDefinition(AnonFunctionDefinition<'ident>),
	PrefixExpression(Box<PrefixExpression<'ident>>),
	TableConstructor(TableConstructor<'ident>),
	InfixOperation(
		Box<Expression<'ident>>,
		InfixOperation,
		Box<Expression<'ident>>,
	),
	PrefixOperation(PrefixOperation, Box<Expression<'ident>>),
}

#[derive(Clone, Debug)]
pub enum PrefixExpression<'ident> {
	Variable(Variable<'ident>),
	FunctionCall(Box<FunctionCall<'ident>>),
	ClosedExpression(Expression<'ident>),
}

#[derive(Clone, Debug)]
pub enum FunctionCall<'ident> {
	CallFunction(PrefixExpression<'ident>, Arguments<'ident>),
	CallObjectFunction(
		PrefixExpression<'ident>,
		Identifier<'ident>,
		Arguments<'ident>,
	),
}

impl<'ident> Into<Statement<'ident>> for FunctionCall<'ident> {
	fn into(self) -> Statement<'ident> {
		Statement::FunctionCall(self)
	}
}

#[derive(Clone, Debug)]
pub enum Arguments<'ident> {
	ClosedExpressionList(Option<ExpressionList<'ident>>),
	TableConstructor(TableConstructor<'ident>),
	LiteralString(LiteralString),
}

#[derive(Clone, Debug)]
pub struct AnonFunctionDefinition<'ident>(pub FunctionBody<'ident>);

#[derive(Clone, Debug)]
pub struct FunctionBody<'ident>(pub Option<ParameterList<'ident>>, pub Block<'ident>);

#[derive(Clone, Debug)]
pub enum ParameterList<'ident> {
	IdentifierList(IdentifierList<'ident>),
	IdentifierListWithVarArgs(IdentifierList<'ident>),
	VarArgs,
}

#[derive(Clone, Debug)]
pub struct TableConstructor<'ident>(pub Option<FieldList<'ident>>);

#[derive(Clone, Debug)]
pub struct FieldList<'ident>(pub Vec<Field<'ident>>);

#[derive(Clone, Debug)]
pub enum Field<'ident> {
	BracketField(Expression<'ident>, Expression<'ident>),
	IdentifierField(Identifier<'ident>, Expression<'ident>),
	Expression(Expression<'ident>),
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
