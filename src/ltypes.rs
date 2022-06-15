//! # Basic Lua Types
//!
//! The types defined in this module are used to describe the sizes and
//! operations used by various lua types. Numbers, floats, strings, tables;
//! All of those types can be found in this module, along with `LuaState`,
//! a part of the native scripting interface.

pub mod object;
pub mod state;
pub mod string;
pub mod func;
pub mod table;

// 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
// the total memory used by Lua (in bytes). Usually, 'size_t' and
// 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
/// Total memory counter size (unsigned)
pub type ULuaMemSize = u32;
/// Total memory counter size (signed)
pub type LuaMemSize = i32;

pub type ULuaByte = u8;
pub type LuaByte = i8;

/// Type for floating points used in Lua
pub type LuaNumber = f32;
/// Type for unsigned integers in Lua
pub type ULuaInteger = u32;
/// Type for integers in Lua
pub type LuaInteger = i32;
