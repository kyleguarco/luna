use crate::{formats::ABC, Props};

struct Move {
	format: ABC,
}

impl Props for Move {
	type Format = ABC;

	fn get_format(&self) -> &Self::Format {
		&self.format
	}

	fn sets_reg_a(&self) -> bool {
		true
	}
}
