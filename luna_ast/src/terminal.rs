//! # Compiler types.
//!
//! These are representations of source structure, which are generated
//! from the parser.

pub type IdentifierList = Vec<Identifier>;

#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Clone, Debug)]
pub enum Numeral {
	Integer(isize),
	Float(f64),
}

#[derive(Clone, Debug)]
pub struct LiteralString(pub String);
