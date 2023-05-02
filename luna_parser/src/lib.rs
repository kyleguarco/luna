mod parse;
mod terminal;

/// The input type for all of the parsers.
pub(crate) type In<'a> = &'a str;

pub use parse::chunk;
