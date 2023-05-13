use misc::ReturnStatement;
use statement::Statement;

pub mod attribute;
pub mod expression;
pub mod function;
pub mod misc;
pub mod operation;
pub mod statement;
pub mod table;
pub mod terminal;
pub mod variable;

#[derive(Clone, Debug)]
pub struct Chunk(pub Block);

#[derive(Clone, Debug)]
pub struct Block {
	/// The statements within this block.
	pub stlist: Vec<Statement>,
	/// The return statement, if any. Void if `None`.
	pub oret: Option<ReturnStatement>,
}
