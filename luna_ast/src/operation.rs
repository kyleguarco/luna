/// Grammar (binop): `'+' | '-' | '*' | '/'
/// | '//' | '^' | '%' | '&' | '~' | '|' | '>>'
/// | '<<' | '..' | '<' | ‘<=' | '>' | ‘>='
/// | '==' | ‘~=' | <and> | <or>`
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperation {
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

/// Grammar (unop): `'-' | <not> | '#' | '~'`
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperation {
	Negate,
	Not,
	Length,
	BitwiseNot,
}
