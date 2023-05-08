mod combinator;
mod parse;
mod terminal;

/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

/// Abbreviated parser result type
pub(crate) type IRes<'a, O> = IResult<In<'a>, O>;

use nom::IResult;
pub use parse::chunk;
