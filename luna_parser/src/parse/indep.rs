//! # Independant parsers
//! These parsers don't recurse into other parsers.

use luna_ast::operation::{InfixOperation, PrefixOperation};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete,
	combinator::{recognize, value},
	IResult,
};

use crate::{
	terminal::{
		keyword::{KAND, KNOT, KOR},
		string::*,
	},
	In,
};

pub fn fieldsep(input: In) -> IResult<In, In> {
	recognize(alt((complete::char(COMMA), complete::char(SEMICOLON))))(input)
}

pub fn binop(input: In) -> IResult<In, InfixOperation> {
	use InfixOperation::*;

	alt((
		value(Add, complete::char(PLUS)),
		value(Subtract, complete::char(MINUS)),
		value(Multiply, complete::char(STAR)),
		value(Divide, complete::char(SLASH)),
		value(FloorDivide, tag(DOUBLESLASH)),
		value(Power, complete::char(CARET)),
		value(Modulo, complete::char(PERCENT)),
		value(BitwiseAnd, complete::char(AMPH)),
		value(BitwiseXor, complete::char(TILDE)),
		value(BitwiseOr, complete::char(PIPE)),
		value(BitwiseRightShift, tag(RSHIFT)),
		value(BitwiseLeftShift, tag(LSHIFT)),
		value(Concat, tag(DOUBLEDOT)),
		value(LessThan, complete::char(LESS)),
		value(LessEqual, tag(LESSEQUAL)),
		value(GreaterThan, complete::char(GREATER)),
		value(GreaterEqual, tag(GREATEREQUAL)),
		value(IsEqual, tag(ISEQUAL)),
		value(IsNotEqual, tag(NOTEQUAL)),
		value(And, tag(KAND)),
		value(Or, tag(KOR)),
	))(input)
}

pub fn unop(input: In) -> IResult<In, PrefixOperation> {
	use PrefixOperation::*;

	alt((
		value(Not, complete::char(MINUS)),
		value(Negate, tag(KNOT)),
		value(Length, complete::char(POUND)),
		value(BitwiseNot, complete::char(TILDE)),
	))(input)
}
