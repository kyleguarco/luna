#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FormatKind {
	ABC = 0,
	ABX = 1,
	ASBX = 2,
	AX = 3,
	ISJ = 4,
}

/// Generic instruction format with three arguments.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ABC {
	pub c: u8,
	pub b: u8,
	pub k: bool,
	pub a: u8,
}

/// Instruction format that's mostly used for loading.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ABX {
	pub bx: u32,
	pub a: u8,
}

/// Instruction format that's mostly used for loading (signed).
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ASBX {
	pub sbx: i32,
	pub a: u8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AX {
	pub ax: u32,
}

/// Instruction format specifically used for jumps.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ISJ {
	pub sj: i32,
}
