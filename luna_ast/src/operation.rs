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
