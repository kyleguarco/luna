use crate::{
	affix::{Index, Prefix, Suffix},
	expression::Value,
	terminal::Name,
};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Name(Name),
	Indexed {
		pfix: Prefix,
		slist: Vec<Suffix>,
		index: Index,
	},
}

impl From<Variable> for Value {
	fn from(value: Variable) -> Self {
		Self::Variable(value)
	}
}
