use crate::types::Statement;

#[derive(Debug)]
pub struct And;

#[derive(Debug)]
pub struct Break;

impl<'a> Into<Statement<'a>> for Break {
	fn into(self) -> Statement<'a> {
		Statement::Break
	}
}

#[derive(Debug)]
pub struct Do;

#[derive(Debug)]
pub struct Else;

#[derive(Debug)]
pub struct ElseIf;

#[derive(Debug)]
pub struct End;

#[derive(Debug)]
pub struct False;

#[derive(Debug)]
pub struct For;

#[derive(Debug)]
pub struct Function;

#[derive(Debug)]
pub struct Goto;

#[derive(Debug)]
pub struct If;

#[derive(Debug)]
pub struct In;

#[derive(Debug)]
pub struct Local;

#[derive(Debug)]
pub struct Nil;

#[derive(Debug)]
pub struct Not;

#[derive(Debug)]
pub struct Or;

#[derive(Debug)]
pub struct Repeat;

#[derive(Debug)]
pub struct Return;

#[derive(Debug)]
pub struct Then;

#[derive(Debug)]
pub struct True;

#[derive(Debug)]
pub struct Until;

#[derive(Debug)]
pub struct While;
