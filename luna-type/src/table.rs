use std::collections::HashMap;

use crate::{gc::GCObject, Value};

pub struct Table {
	htable: HashMap<GCObject, GCObject>,
}
