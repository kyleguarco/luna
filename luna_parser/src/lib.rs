use luna_ast::types::Chunk;
use nom::Finish;
use parse::block;

#[cfg(test)]
mod test;

mod combinator;
pub mod parse;
pub mod terminal;

/// Grammar: `chunk ::= block`
pub fn chunk(input: &str) -> Result<Chunk, &str> {
	block(input)
		.finish()
		.map(|(_, bl)| Chunk(bl))
		.map_err(|e| e.input)
}
