use crate::terminal::{
	identifier,
	keyword::{keyword, Keyword},
	string::{
		AMPHERSAND, CARET, COMMA, DOUBLECOLON, DOUBLEDOT, DOUBLESLASH, GREATER, GREATEREQUAL,
		ISEQUAL, LESS, LESSEQUAL, MINUS, NOTEQUAL, OCTOTHORPE, PERCENT, PIPE, PLUS, SEMICOLON,
		SHIFTLEFT, SHIFTRIGHT, SLASH, STAR, TILDE,
	},
};
use luna_ast::types::{
	AnonFunctionDefinition, Arguments, Attribute, AttributeName, AttributeNameList, Block,
	Expression, ExpressionList, Field, FieldList, FunctionBody, FunctionCall, FunctionIdentifier,
	Identifier, IdentifierList, InfixOperation, Label, ParameterList, PrefixExpression,
	PrefixOperation, ReturnStatement, TableConstructor, Variable, VariableList,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{opt, value},
	multi::{many0, many1},
	sequence::{delimited, terminated},
	IResult,
};

pub use stat::stat;

mod stat;

pub fn block(input: &str) -> IResult<&str, Block> {
	let (input, slist) = many0(stat)(input)?;
	let (input, ret) = opt(retstat)(input)?;
	Ok((input, Block(slist, ret)))
}

pub fn attnamelist(input: &str) -> IResult<&str, AttributeNameList> {
	fn inner(input: &str) -> IResult<&str, AttributeName> {
		let (input, ident) = identifier(input)?;
		let (input, attr) = attrib(input)?;
		Ok((input, AttributeName(ident, attr)))
	}

	let (input, alist) = many1(terminated(inner, tag(COMMA)))(input)?;
	Ok((input, AttributeNameList(alist)))
}

pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	let (input, attr) = opt(delimited(tag(LESS), identifier, tag(LESS)))(input)?;
	Ok((input, Attribute(attr)))
}

pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	let (input, _) = keyword(Keyword::Return)(input)?;
	let (input, elist) = opt(explist)(input)?;
	let (input, _) = opt(tag(SEMICOLON))(input)?;
	Ok((input, ReturnStatement(elist)))
}

pub fn label(input: &str) -> IResult<&str, Label> {
	let (input, _) = tag(DOUBLECOLON)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, _) = tag(DOUBLECOLON)(input)?;

	Ok((input, Label(ident)))
}

pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	let (input, ident) = identifier(input)?;
	todo!()
}

pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	todo!()
}

pub fn var(input: &str) -> IResult<&str, Variable> {
	todo!()
}

pub fn namelist(input: &str) -> IResult<&str, IdentifierList> {
	let (input, ilist) = many1(terminated(identifier, tag(COMMA)))(input)?;
	Ok((input, IdentifierList(ilist)))
}

pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	let (input, elist) = many1(terminated(exp, tag(COMMA)))(input)?;
	Ok((input, ExpressionList(elist)))
}

pub fn exp(input: &str) -> IResult<&str, Expression> {
	todo!()
}

pub fn prefixexp(input: &str) -> IResult<&str, PrefixExpression> {
	todo!()
}

pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	todo!()
}

pub fn args(input: &str) -> IResult<&str, Arguments> {
	todo!()
}

pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	todo!()
}

pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	todo!()
}

pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	todo!()
}

pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	todo!()
}

pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	todo!()
}

pub fn field(input: &str) -> IResult<&str, Field> {
	todo!()
}

pub fn fieldsep(input: &str) -> IResult<&str, &str> {
	alt((tag(COMMA), tag(SEMICOLON)))(input)
}

pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	alt((
		value(InfixOperation::Add, tag(PLUS)),
		value(InfixOperation::Subtract, tag(MINUS)),
		value(InfixOperation::Multiply, tag(STAR)),
		value(InfixOperation::Divide, tag(SLASH)),
		value(InfixOperation::FloorDivide, tag(DOUBLESLASH)),
		value(InfixOperation::Power, tag(CARET)),
		value(InfixOperation::Modulo, tag(PERCENT)),
		value(InfixOperation::BitwiseAnd, tag(AMPHERSAND)),
		value(InfixOperation::BitwiseXor, tag(TILDE)),
		value(InfixOperation::BitwiseOr, tag(PIPE)),
		value(InfixOperation::BitwiseRightShift, tag(SHIFTRIGHT)),
		value(InfixOperation::BitwiseLeftShift, tag(SHIFTLEFT)),
		value(InfixOperation::Concat, tag(DOUBLEDOT)),
		value(InfixOperation::LessThan, tag(LESS)),
		value(InfixOperation::LessEqual, tag(LESSEQUAL)),
		value(InfixOperation::GreaterThan, tag(GREATER)),
		value(InfixOperation::GreaterEqual, tag(GREATEREQUAL)),
		value(InfixOperation::IsEqual, tag(ISEQUAL)),
		value(InfixOperation::IsNotEqual, tag(NOTEQUAL)),
		value(InfixOperation::And, keyword(Keyword::And)),
		value(InfixOperation::Or, keyword(Keyword::Or)),
	))(input)
}

pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	alt((
		value(PrefixOperation::Not, tag(MINUS)),
		value(PrefixOperation::Negate, keyword(Keyword::Not)),
		value(PrefixOperation::Length, tag(OCTOTHORPE)),
		value(PrefixOperation::BitwiseNot, tag(TILDE)),
	))(input)
}
