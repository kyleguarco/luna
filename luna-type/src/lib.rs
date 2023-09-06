pub use number::{Float, Integer, Number};

mod number;

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
	Nil,
	Boolean,
	Number,
	String,
	Function,
	Userdata,
	Thread,
	Table,
}

impl Kind {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Nil => "nil",
			Self::Boolean => "boolean",
			Self::Number => "number",
			Self::String => "string",
			Self::Function => "function",
			Self::Userdata => "userdata",
			Self::Thread => "thread",
			Self::Table => "table",
		}
	}
}

/// A Lua value.
///
/// Types implementing this trait can be manipulated inside a Lua state.
pub trait Value: Default {
	/// The Lua type `Self` represents.
	///
	/// The provided methods from this trait rely on the correctness of this implementation.
	fn kind(&self) -> Kind;

	/// Map `Self` to another `Value`
	fn map<F, V>(self, f: F) -> V
	where
		F: FnOnce(Self) -> V,
		V: Value,
	{
		f(self)
	}

	/// Map `Self` to another `Value`, with a possibility to return nothing.
	fn map_opt<F, V>(self, f: F) -> Option<V>
	where
		F: FnOnce(Self) -> Option<V>,
		V: Value,
	{
		f(self)
	}

	fn is_nil(&self) -> bool {
		self.kind() == Kind::Nil
	}

	fn is_boolean(&self) -> bool {
		self.kind() == Kind::Boolean
	}

	fn is_number(&self) -> bool {
		self.kind() == Kind::Number
	}

	fn is_string(&self) -> bool {
		self.kind() == Kind::String
	}

	fn is_function(&self) -> bool {
		self.kind() == Kind::Function
	}
}

impl Value for bool {
	fn kind(&self) -> Kind {
		Kind::Boolean
	}
}

impl<V: Value> Value for Option<V> {
	fn kind(&self) -> Kind {
		match self {
			Some(value) => value.kind(),
			None => Kind::Nil,
		}
	}
}
