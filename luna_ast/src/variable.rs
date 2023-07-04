use crate::{
	affix::{Affix, Index},
	expression::Value,
	terminal::Name,
};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Name(Name),
	Indexed { affix: Affix, index: Index },
}

impl From<Variable> for Value {
	fn from(value: Variable) -> Self {
		Self::Variable(value)
	}
}
