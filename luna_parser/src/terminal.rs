use luna_ast::types::Identifier;
use nom::IResult;

use crate::In;

pub mod keyword;
pub mod string;

pub fn identifier(input: In) -> IResult<In, Identifier> {
	todo!()
}
