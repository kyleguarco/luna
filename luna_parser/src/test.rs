use luna_ast::{statement::Statement, Block, Chunk, ReturnStatement};

mod block;
mod function;

pub(self) fn test_chunk(stlist: Vec<Statement>, oret: Option<ReturnStatement>) -> Chunk {
	Chunk(Block { stlist, oret })
}
