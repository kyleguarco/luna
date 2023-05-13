//! # Compiler types.
//!
//! These are representations of source structure, which are generated
//! from the parser.

use crate::{expression::Expression, variable::Variable};

pub type IdentifierList = Vec<Identifier>;

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

impl From<Identifier> for Variable {
	fn from(value: Identifier) -> Self {
		Self::Identifier(value)
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
