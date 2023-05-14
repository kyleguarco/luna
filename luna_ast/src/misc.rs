use crate::{expression::ExpressionList, statement::Statement, terminal::Name};

#[derive(Clone, Debug)]
pub struct Label(pub Name);

impl From<Label> for Statement {
	fn from(value: Label) -> Self {
		Self::Label(value)
	}
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
	pub oelist: Option<ExpressionList>,
}
