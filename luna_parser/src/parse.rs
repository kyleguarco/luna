//! # Non-recursive Parsers
//! These parsers don't rely on recursion to function.

use luna_ast::{
	attribute::Attribute,
	expression::{Expression, PrefixExpression, ReturnStatement},
	function::{FunctionBody, FunctionCall, FunctionIdentifier},
	operation::{InfixOperation, PrefixOperation},
	statement::{FunctionDefinition, Statement},
	types::{Arguments, Block, Chunk, Field, FieldList, Label, ParameterList, TableConstructor},
	variable::Variable,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char as tchar,
	combinator::{map, opt, recognize, value},
	multi::{many0, separated_list1},
	sequence::{delimited, pair, preceded, separated_pair, terminated},
};

use crate::{
	combinator::{assign, braces, bracket, list, paren},
	terminal::{
		identifier,
		keyword::{KAND, KNOT, KOR, KRETURN},
		string::*,
	},
	IRes, In,
};

pub fn chunk(input: In) -> Result<Chunk, ()> {
	todo!()
}

pub(crate) fn block(input: In) -> IRes<Block> {
	map(pair(many0(stat), opt(retstat)), Block::from_parser)(input)
}

pub(crate) fn stat(input: In) -> IRes<Statement> {
	todo!()
}

pub(crate) fn attrib(input: In) -> IRes<Attribute> {
	map(
		opt(delimited(tchar(LESS), identifier, tchar(GREATER))),
		Attribute,
	)(input)
}

pub(crate) fn retstat(input: In) -> IRes<ReturnStatement> {
	map(
		delimited(tag(KRETURN), opt(list(exp)), tchar(SEMICOLON)),
		ReturnStatement,
	)(input)
}

pub(crate) fn label(input: In) -> IRes<Label> {
	map(
		delimited(tag(DOUBLECOLON), identifier, tag(DOUBLECOLON)),
		Label,
	)(input)
}

pub(crate) fn funcname(input: In) -> IRes<FunctionIdentifier> {
	map(
		pair(
			separated_list1(tchar(DOT), identifier),
			opt(preceded(tchar(COLON), identifier)),
		),
		FunctionIdentifier::from_parser,
	)(input)
}

#[inline(always)]
pub(crate) fn var(input: In) -> IRes<Variable> {
	use Variable::*;

	alt((
		map(identifier, Identifier),
		map(pair(prefixexp, bracket(exp)), |(pexp, exp)| {
			PrefixExpressionIndex(Box::new(pexp), Box::new(exp))
		}),
		map(
			separated_pair(prefixexp, tchar(DOT), identifier),
			|(pexp, ident)| PrefixExpressionIdentifier(Box::new(pexp), ident),
		),
	))(input)
}

pub(crate) fn exp(input: In) -> IRes<Expression> {
	todo!()
}

pub(crate) fn prefixexp(input: In) -> IRes<PrefixExpression> {
	use PrefixExpression::*;

	alt((
		map(var, Variable),
		map(functioncall, |fcall| FunctionCall(Box::new(fcall))),
		map(paren(exp), ClosedExpression),
	))(input)
}

pub(crate) fn functioncall(input: In) -> IRes<FunctionCall> {
	todo!()
}

pub(crate) fn args(input: In) -> IRes<Arguments> {
	todo!()
}

pub(crate) fn functiondef(input: In) -> IRes<FunctionDefinition> {
	todo!()
}

pub(crate) fn funcbody(input: In) -> IRes<FunctionBody> {
	todo!()
}

pub(crate) fn parlist(input: In) -> IRes<ParameterList> {
	use ParameterList::*;

	alt((
		map(
			separated_pair(list(identifier), tchar(COMMA), tag(TRIPLEDOT)),
			|(ilist, _)| IdentifierListWithVarArgs(ilist),
		),
		map(list(identifier), IdentifierList),
		value(VarArgs, tag(TRIPLEDOT)),
	))(input)
}

pub(crate) fn tableconstructor(input: In) -> IRes<TableConstructor> {
	map(braces(opt(fieldlist)), TableConstructor)(input)
}

pub(crate) fn fieldlist(input: In) -> IRes<FieldList> {
	terminated(separated_list1(fieldsep, field), opt(fieldsep))(input)
}

pub(crate) fn field(input: In) -> IRes<Field> {
	use Field::*;

	alt((
		map(assign(bracket(exp), exp), BracketField),
		map(assign(identifier, exp), IdentifierField),
		map(exp, Expression),
	))(input)
}

pub(crate) fn fieldsep(input: In) -> IRes<In> {
	recognize(alt((tchar(COMMA), tchar(SEMICOLON))))(input)
}

pub(crate) fn binop(input: In) -> IRes<InfixOperation> {
	use InfixOperation::*;

	alt((
		value(Add, tchar(PLUS)),
		value(Subtract, tchar(MINUS)),
		value(Multiply, tchar(STAR)),
		value(Divide, tchar(SLASH)),
		value(FloorDivide, tag(DOUBLESLASH)),
		value(Power, tchar(CARET)),
		value(Modulo, tchar(PERCENT)),
		value(BitwiseAnd, tchar(AMPH)),
		value(BitwiseXor, tchar(TILDE)),
		value(BitwiseOr, tchar(PIPE)),
		value(BitwiseRightShift, tag(RSHIFT)),
		value(BitwiseLeftShift, tag(LSHIFT)),
		value(Concat, tag(DOUBLEDOT)),
		value(LessThan, tchar(LESS)),
		value(LessEqual, tag(LESSEQUAL)),
		value(GreaterThan, tchar(GREATER)),
		value(GreaterEqual, tag(GREATEREQUAL)),
		value(IsEqual, tag(ISEQUAL)),
		value(IsNotEqual, tag(NOTEQUAL)),
		value(And, tag(KAND)),
		value(Or, tag(KOR)),
	))(input)
}

pub(crate) fn unop(input: In) -> IRes<PrefixOperation> {
	use PrefixOperation::*;

	alt((
		value(Not, tchar(MINUS)),
		value(Negate, tag(KNOT)),
		value(Length, tchar(POUND)),
		value(BitwiseNot, tchar(TILDE)),
	))(input)
}
