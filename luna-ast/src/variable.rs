use crate::{affix::Affix, expression::Value, terminal::Name};

pub type VariableList = Vec<Variable>;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Name(Name),
	Affixed(Affix),
}

impl From<Variable> for Value {
	fn from(value: Variable) -> Self {
		Self::Variable(value)
	}
}
