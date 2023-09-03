use crate::Value;

pub struct State<Tags> {
	stack: Vec<Tags>,
}

impl<Tags: Value> State<Tags> {
	pub fn new() -> Self {
		Self {
			stack: Vec::new()
		}
	}

	pub fn push(&mut self, value: impl Into<Tags>) {
		self.stack.push(value.into())
	}

	pub fn pop(&mut self) -> Option<Tags> {
		self.stack.pop()
	}
}
