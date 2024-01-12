//! ## `OpMode` Implementation

use crate::{cn, formats::FormatKind};

use super::OpMode;

impl OpMode {
	/// Specifies flags in a raw operation mode byte.
	///
	/// # Arguments
	///
	/// * `mm` - Calls a metamethod.
	/// * `ot` - Sets the top of the stack for the next instruction (out top).
	/// * `it` - Uses the top of the stack (in top).
	/// * `t` - Next instruction must be a jump (this is a test).
	/// * `a` - Sets the `A` register.
	pub(super) const fn new(
		mm: bool, ot: bool, it: bool, t: bool, a: bool, format: FormatKind,
	) -> Self {
		Self(
			((mm as u8) << 7)
				| ((ot as u8) << 6)
				| ((it as u8) << 5)
				| ((t as u8) << 4)
				| ((a as u8) << 3)
				| (format as u8),
		)
	}

	pub fn format(&self) -> Option<FormatKind> {
		match self.0 & 7 {
			cn::FMT_ABC => Some(FormatKind::ABC),
			cn::FMT_ABX => Some(FormatKind::ABX),
			cn::FMT_ASBX => Some(FormatKind::ASBX),
			cn::FMT_AX => Some(FormatKind::AX),
			cn::FMT_ISJ => Some(FormatKind::ISJ),
			_ => None,
		}
	}

	pub unsafe fn format_unchecked(&self) -> FormatKind {
		unsafe { self.format().unwrap_unchecked() }
	}

	pub fn sets_reg_a(&self) -> bool {
		(self.0 & (1 << 3)) != 0
	}

	pub fn is_test(&self) -> bool {
		(self.0 & (1 << 4)) != 0
	}

	pub fn uses_top(&self) -> bool {
		(self.0 & (1 << 5)) != 0
	}

	pub fn sets_top(&self) -> bool {
		(self.0 & (1 << 6)) != 0
	}

	pub fn calls_metamethod(&self) -> bool {
		(self.0 & (1 << 7)) != 0
	}
}
