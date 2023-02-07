use luna_ast::types::Chunk;
use nom::Finish;
use parse::block;

mod combinator;
pub mod parse;
pub mod terminal;

#[cfg(test)]
mod test;

pub fn chunk(input: &str) -> Result<Chunk, &str> {
	let (input, block) = block(input).finish().map_err(|e| e.input)?;
	assert!(input.is_empty());
	Ok(Chunk(block))
}
