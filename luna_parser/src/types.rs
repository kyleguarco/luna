///! Compiler types.
///!
///! These are representations of source structure, which are generated
///! from the parser.

struct Identifier<'a>(&'a str);

struct Numeral;

struct LiteralString;

struct Chunk;

struct Block;

enum Statement {
	Definition,
	FunctionCall,
	Break,
	Goto,
	Do,
	While,
	RepeatUntil,
	IfTree,
	For,
	FunctionDefinition,
	LocalFunctionDefinition,
	LocalDefinitionWithAttribute,
}

struct AttributeNameList;

struct Attribute;

struct ReturnStatement;

struct Label;

struct FunctionIdentifier;

struct VariableList;

enum Variable {
	Identifier,
	PrefixExpressionIndex,
	PrefixExpressionIdentifier,
}

struct IdentifierList;

struct ExpressionList;

enum Expression {
	Nil,
	False,
	True,
	Numeral,
	LiteralString,
	TripleDots,
	FunctionDefinition,
	PrefixExpression,
	TableConstructor,
	InfixOperation,
	PrefixOperation,
}

enum PrefixExpression {
	Variable,
	FunctionCall,
	ClosedExpression,
}

enum FunctionCall {
	CallFunction,
	CallObjectFunction,
}

enum Arguments {
	ClosedExpressionList,
	TableConstructor,
	LiteralString,
}

struct FunctionDefinition;

struct FunctionBody;

enum ParameterList {
	IdentifierList,
	IdentifierListTripleDots,
	TripleDots,
}

struct TableConstructor;

struct FieldList;

enum Field {
	BracketExpression,
	IdentifierExpression,
	Expression,
}

enum InfixOperation {
	Add,
	Substract,
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

enum PrefixOperation {
	Negate,
	Not,
	Length,
	BitwiseNot,
}
