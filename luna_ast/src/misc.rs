use crate::{expression::ExpressionList, statement::Statement, terminal::Identifier};

#[derive(Clone, Debug)]
pub struct Label(pub Identifier);

impl From<Label> for Statement {
	fn from(val: Label) -> Self {
		Self::Label(val)
	}
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
	pub oelist: Option<ExpressionList>,
}
