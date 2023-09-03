pub use state::State;

mod state;
mod number;

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
	fn kind(&self) -> Kind;
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
