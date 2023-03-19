///! Compiler types.
///!
///! These are representations of source structure, which are generated
///! from the parser.
use self::{
	attribute::AttributeName,
	expression::{Expression, ExpressionList, ReturnStatement},
	function::FunctionBody,
	statement::Statement,
};

pub mod attribute;
pub mod expression;
pub mod function;
pub mod operation;
pub mod statement;
pub mod variable;

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
pub struct Block(pub Vec<Statement>, pub Option<ReturnStatement>);

#[derive(Clone, Debug)]
pub struct AttributeNameList(pub Vec<AttributeName>);

#[derive(Clone, Debug)]
pub struct Label(pub Identifier);

impl Into<Statement> for Label {
	fn into(self) -> Statement {
		Statement::Label(self)
	}
}

#[derive(Clone, Debug)]
pub struct IdentifierList(pub Vec<Identifier>);

#[derive(Clone, Debug)]
pub enum Arguments {
	ClosedExpressionList(Option<ExpressionList>),
	TableConstructor(TableConstructor),
	LiteralString(LiteralString),
}

#[derive(Clone, Debug)]
pub struct AnonFunctionDefinition(pub FunctionBody);

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
