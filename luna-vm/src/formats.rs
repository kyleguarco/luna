use crate::{inst::Instruction, ops::code::Format};

pub type ASize = u8;
pub type AXSize = u32;
pub type BSize = u8;
pub type BXSize = u32;
pub type SBXSize = i32;
pub type CSize = u8;
pub type KSize = bool;
pub type SJSize = i32;

#[derive(Debug, Clone, Copy)]
pub struct InvalidFormatError;

/// Decodes `Self` from an [Instruction].
pub(crate) trait DecodableFormat: Sized {
	/// Transforms a raw [Instruction] into `Self`.
	///
	/// # Errors
	///
	/// The instruction may not be of the correct format. Return
	/// `None` if this is the case.
	fn decode_format(inst: Instruction) -> Option<Self>;
}

impl<F> DecodableFormat for &F
where
	F: DecodableFormat,
{
	fn decode_format(inst: Instruction) -> Option<Self> {
		Self::decode_format(inst)
	}
}

/// Generic instruction format with three arguments.
pub type ABC = (CSize, BSize, KSize, ASize);

impl DecodableFormat for ABC {
	fn decode_format(inst: Instruction) -> Option<Self> {
		todo!()
	}
}

impl TryFrom<Format> for ABC {
	type Error = InvalidFormatError;

	fn try_from(fmt: Format) -> Result<Self, Self::Error> {
		match fmt {
			Format::ABC(abc) => Ok(abc),
			_ => Err(InvalidFormatError),
		}
	}
}

/// Instruction format that's mostly used for loading.
pub type ABX = (BXSize, ASize);

impl DecodableFormat for ABX {
	fn decode_format(inst: Instruction) -> Option<Self> {
		todo!()
	}
}

impl TryFrom<Format> for ABX {
	type Error = InvalidFormatError;

	fn try_from(fmt: Format) -> Result<Self, Self::Error> {
		match fmt {
			Format::ABX(abx) => Ok(abx),
			_ => Err(InvalidFormatError),
		}
	}
}

/// Instruction format that's mostly used for loading (signed).
pub type ASBX = (SBXSize, ASize);

impl DecodableFormat for ASBX {
	fn decode_format(inst: Instruction) -> Option<Self> {
		todo!()
	}
}

impl TryFrom<Format> for ASBX {
	type Error = InvalidFormatError;

	fn try_from(fmt: Format) -> Result<Self, Self::Error> {
		match fmt {
			Format::ASBX(asbx) => Ok(asbx),
			_ => Err(InvalidFormatError),
		}
	}
}

pub type AX = AXSize;

impl DecodableFormat for AX {
	fn decode_format(inst: Instruction) -> Option<Self> {
		todo!()
	}
}

impl TryFrom<Format> for AX {
	type Error = InvalidFormatError;

	fn try_from(fmt: Format) -> Result<Self, Self::Error> {
		match fmt {
			Format::AX(ax) => Ok(ax),
			_ => Err(InvalidFormatError),
		}
	}
}

/// Instruction format specifically used for jumps.
pub type ISJ = SJSize;

impl DecodableFormat for ISJ {
	fn decode_format(inst: Instruction) -> Option<Self> {
		todo!()
	}
}

impl TryFrom<Format> for ISJ {
	type Error = InvalidFormatError;

	fn try_from(fmt: Format) -> Result<Self, Self::Error> {
		match fmt {
			Format::ISJ(isj) => Ok(isj),
			_ => Err(InvalidFormatError),
		}
	}
}
