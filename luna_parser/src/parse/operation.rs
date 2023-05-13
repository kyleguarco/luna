use luna_ast::operation::{InfixOperation, PrefixOperation};
use nom::{
	branch::alt, bytes::complete::tag, character::complete::char as tchar, combinator::value,
};

use crate::{
	terminal::{keyword::*, string::*},
	IRes, In,
};

pub fn infix_op(input: In) -> IRes<InfixOperation> {
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

pub fn prefix_op(input: In) -> IRes<PrefixOperation> {
	use PrefixOperation::*;

	alt((
		value(Not, tchar(MINUS)),
		value(Negate, tag(KNOT)),
		value(Length, tchar(POUND)),
		value(BitwiseNot, tchar(TILDE)),
	))(input)
}
