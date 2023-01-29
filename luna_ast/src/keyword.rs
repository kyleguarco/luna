use crate::types::Statement;

#[derive(Debug)]
pub enum Keyword {
	And,
	Break,
	Do,
	Else,
	ElseIf,
	End,
	False,
	For,
	Function,
	Goto,
	If,
	In,
	Local,
	Nil,
	Not,
	Or,
	Repeat,
	Return,
	Then,
	True,
	Until,
	While,
}

impl<'a> TryInto<Statement<'a>> for Keyword {
	type Error = ();

	fn try_into(self) -> Result<Statement<'a>, Self::Error> {
		match self {
			Self::Break => Ok(Statement::Break),
			_ => Err(())
		}
	}
}
