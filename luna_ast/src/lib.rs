use types::{FunctionCall, Label, Statement};

pub mod types;
pub mod keyword;

impl<'a> Into<Statement<'a>> for Label<'a> {
	fn into(self) -> Statement<'a> {
		Statement::Label(self)
	}
}

impl<'a> Into<Statement<'a>> for FunctionCall {
	fn into(self) -> Statement<'a> {
		Statement::FunctionCall(self)
	}
}
