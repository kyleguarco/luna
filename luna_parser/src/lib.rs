use luna_ast::types::Chunk;
use nom::Finish;
use parse::block;

mod combinator;
pub mod parse;
pub mod terminal;

pub fn chunk(input: &str) -> Result<Chunk, &str> {
	block(input)
		.finish()
		.map(|(_, bl)| Chunk(bl))
		.map_err(|e| e.input)
}
