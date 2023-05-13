use expression::ReturnStatement;
use statement::Statement;
use terminal::Identifier;

pub mod terminal;
pub mod attribute;
pub mod expression;
pub mod function;
pub mod operation;
pub mod statement;
pub mod variable;
pub mod table;

#[derive(Clone, Debug)]
pub struct Chunk(pub Block);

#[derive(Clone, Debug)]
pub struct Block {
	/// The statements within this block.
	pub stlist: Vec<Statement>,
	/// The return statement, if any. Void if `None`.
	pub oret: Option<ReturnStatement>,
}

#[derive(Clone, Debug)]
pub struct Label(pub Identifier);

impl From<Label> for Statement {
	fn from(val: Label) -> Self {
		Statement::Label(val)
	}
}

