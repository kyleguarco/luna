//! # Compiler types.
//!
//! These are representations of source structure, which are generated
//! from the parser.

use crate::{expression::Expression, function::Arguments, variable::Variable};

pub type NameList = Vec<Name>;

#[derive(Clone, Debug)]
pub struct Name(pub String);

impl From<Name> for Variable {
	fn from(value: Name) -> Self {
		Self::Name(value)
	}
}

#[derive(Clone, Debug)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

impl From<Numeral> for Expression {
	fn from(value: Numeral) -> Self {
		Self::Numeral(value)
	}
}

#[derive(Clone, Debug)]
pub struct LiteralString(pub String);

impl From<LiteralString> for Expression {
	fn from(value: LiteralString) -> Self {
		Self::LiteralString(value)
	}
}

impl From<LiteralString> for Arguments {
	fn from(value: LiteralString) -> Self {
		Self::LiteralString(value)
	}
}
