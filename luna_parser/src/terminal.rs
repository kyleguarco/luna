use luna_ast::terminal::{Identifier, Numeral, LiteralString};

use crate::{In, IRes};

pub mod keyword;
pub mod string;

pub fn identifier(input: In) -> IRes<Identifier> {
	todo!()
}

pub fn numeral(input: In) -> IRes<Numeral> {
	todo!()
}

pub fn literal_string(input: In) -> IRes<LiteralString> {
	todo!()
}
