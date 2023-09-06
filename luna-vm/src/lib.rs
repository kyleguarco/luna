mod formats;
mod instrs;

type InstructionSize = u32;

#[repr(u8)]
#[derive(Clone, Copy)]
enum ModeId {
	ABC = 0,
	ABX = 1,
	ASBX = 2,
	AX = 3,
	ISJ = 4,
}

trait Mode {
	fn mode_id() -> ModeId;
}

/// Properties for an instruction opcode as layed out in the original source.
trait Props: Sized {
	type Format: Mode;

	fn get_format(&self) -> &Self::Format;

	fn sets_reg_a(&self) -> bool {
		false
	}

	fn is_test(&self) -> bool {
		false
	}

	fn uses_stack_top(&self) -> bool {
		false
	}

	fn sets_stack_top(&self) -> bool {
		false
	}

	fn is_metamethod(&self) -> bool {
		false
	}

	fn opcode(&self) -> u8 {
		((self.is_metamethod() as u8) << 7)
			| ((self.sets_stack_top() as u8) << 6)
			| ((self.uses_stack_top() as u8) << 5)
			| ((self.is_test() as u8) << 4)
			| ((self.sets_reg_a() as u8) << 3)
			| (Self::Format::mode_id() as u8)
	}
}

/// A trait for types that encode/decode to a 32-bit Lua instruction.
trait Instruction: Props {
	/// Converts the fields from [Props::Format] into a 32-bit VM instruction.
	///
	/// Faillable. `serialize` will fail if any fields are bigger than the instruction set specify.
	fn serialize(self) -> Option<InstructionSize>;
	fn deserialize(inst: InstructionSize) -> Self::Format;
}
