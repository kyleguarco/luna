use std::sync::{Arc, Mutex, Weak};

use crate::value::Value;

type Object<'obj> = Mutex<dyn Value + 'obj>;

#[derive(Default)]
pub struct GCObject<'obj> {
	inner: Arc<Option<Object<'obj>>>,
	next: Option<GCWeak<'obj>>,
}

impl<'obj> GCObject<'obj> {
	pub fn new(obj: impl Value + 'obj) -> Self {
		Self {
			inner: Some(Arc::new(obj)),
			next: None,
		}
	}

	/// Constructs a new GCObject with an empty value.
	///
	/// Equivalent to `GCObject::default()`
	pub fn nil() -> Self {
		Self { inner: None, next: None }
	}
}
