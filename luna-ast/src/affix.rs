use crate::{expression::Expression, function::Arguments, terminal::Name};

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
	ParenExpression(Expression),
	Name(Name),
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
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
#[repr(u8)]
pub enum Suffix {
	Call(Call),
	Index(Index),
}

impl Suffix {
	pub fn to_u8(&self) -> u8 {
		match self {
			Suffix::Call(_) => Suffix::Call as u8,
			Suffix::Index(_) => Suffix::Index as u8,
		}
	}
}

/// A series of names, beginning with a [Name] or [Expression].
#[derive(Clone, Debug, PartialEq)]
pub struct Affix {
	pub pfix: Prefix,
	pub suflist: Vec<Suffix>,
}
