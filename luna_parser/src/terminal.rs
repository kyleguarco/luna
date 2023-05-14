use luna_ast::terminal::{Name, Numeral, LiteralString};

use crate::{In, IRes};

pub mod keyword;
pub mod string;

pub fn name(input: In) -> IRes<Name> {
	todo!()
}

pub fn numeral(input: In) -> IRes<Numeral> {
	todo!()
}

pub fn literal_string(input: In) -> IRes<LiteralString> {
	todo!()
}
