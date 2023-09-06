use crate::{Instruction, InstructionSize, Mode, ModeId, Props};

/// Generic instruction format with three arguments.
#[derive(Default)]
pub(crate) struct ABC {
	pub c: u8,
	pub b: u8,
	pub k: bool,
	pub a: u8,
}

impl Mode for ABC {
	fn mode_id() -> ModeId {
		ModeId::ABC
	}
}

impl<P> Instruction for P
where
	P: Props<Format = ABC>,
{
	fn serialize(self) -> Option<InstructionSize> {
		let ABC { c, b, k, a } = *self.get_format();

		Some(
			(self.opcode() as u32)
				| ((a as u32) << 7)
				| ((b as u32) << 16)
				| ((c as u32) << 24)
				| ((k as u32) << 15),
		)
	}

	fn deserialize(inst: InstructionSize) -> Self::Format {
		todo!()
	}
}

#[derive(Default)]
pub(crate) struct ABX {
	bx: u32,
	a: u8,
}

impl Mode for ABX {
	fn mode_id() -> ModeId {
		ModeId::ABX
	}
}

#[derive(Default)]
pub(crate) struct ASBX {
	sbx: i32,
	a: u8,
}

impl Mode for ASBX {
	fn mode_id() -> ModeId {
		ModeId::ASBX
	}
}

#[derive(Default)]
pub(crate) struct AX {
	ax: u32,
}

impl Mode for AX {
	fn mode_id() -> ModeId {
		ModeId::AX
	}
}

#[derive(Default)]
pub(crate) struct ISJ {
	sj: i32,
}

impl Mode for ISJ {
	fn mode_id() -> ModeId {
		ModeId::ISJ
	}
}
