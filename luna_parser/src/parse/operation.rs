use luna_ast::operation::{BinaryOperation, UnaryOperation};
use nom::{branch::alt, combinator::value};

use crate::{
	combinator::{wschar, wstag},
	terminal::{keyword::*, string::*},
	IRes, In,
};

pub fn binop(input: In) -> IRes<BinaryOperation> {
	dbg!(input);
	use BinaryOperation::*;

	alt((
		value(Add, wschar(PLUS)),
		value(Subtract, wschar(MINUS)),
		value(Multiply, wschar(STAR)),
		value(Divide, wschar(SLASH)),
		value(FloorDivide, wstag(DOUBLESLASH)),
		value(Power, wschar(CARET)),
		value(Modulo, wschar(PERCENT)),
		value(BitwiseAnd, wschar(AMPH)),
		value(BitwiseXor, wschar(TILDE)),
		value(BitwiseOr, wschar(PIPE)),
		value(BitwiseRightShift, wstag(RSHIFT)),
		value(BitwiseLeftShift, wstag(LSHIFT)),
		value(Concat, wstag(DOUBLEDOT)),
		value(LessThan, wschar(LESS)),
		value(LessEqual, wstag(LESSEQUAL)),
		value(GreaterThan, wschar(GREATER)),
		value(GreaterEqual, wstag(GREATEREQUAL)),
		value(IsEqual, wstag(ISEQUAL)),
		value(IsNotEqual, wstag(NOTEQUAL)),
		value(And, wstag(KAND)),
		value(Or, wstag(KOR)),
	))(input)
}

pub fn unop(input: In) -> IRes<UnaryOperation> {
	dbg!(input);
	use UnaryOperation::*;

	alt((
		value(Not, wschar(MINUS)),
		value(Negate, wstag(KNOT)),
		value(Length, wschar(POUND)),
		value(BitwiseNot, wschar(TILDE)),
	))(input)
}
