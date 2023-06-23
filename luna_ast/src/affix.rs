use crate::{expression::Expression, function::Arguments, terminal::Name};

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
	BracedExpression(Expression),
	Name(Name),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Index {
	/// Index using the `'[' exp ']'` syntax
	Expression(Expression),
	/// Index using the `'.' Name` syntax
	Member(Name),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
	/// If this member is `Some`, the function is defined and called as a method
	pub oname: Option<Name>,
	/// Function arguments
	pub argu: Arguments,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Suffix {
	Call(Call),
	Index(Index),
}
