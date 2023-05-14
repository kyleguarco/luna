use luna_ast::terminal::{LiteralString, Name, Numeral};

use crate::{IRes, In};

pub mod keyword;
pub mod string;

pub(crate) fn name(input: In) -> IRes<Name> {
	todo!()
}

pub(crate) fn numeral(input: In) -> IRes<Numeral> {
	todo!()
}

pub(crate) fn literal_string(input: In) -> IRes<LiteralString> {
	todo!()
}
