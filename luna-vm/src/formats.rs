pub type ASize = u8;
pub type AXSize = u32;
pub type BSize = u8;
pub type BXSize = u32;
pub type SBXSize = i32;
pub type CSize = u8;
pub type KSize = bool;
pub type SJSize = i32;

/// Generic instruction format with three arguments.
pub type ABC = (CSize, BSize, KSize, ASize);

/// Instruction format that's mostly used for loading.
pub type ABX = (BXSize, ASize);

/// Instruction format that's mostly used for loading (signed).
pub type ASBX = (SBXSize, ASize);

pub type AX = AXSize;

/// Instruction format specifically used for jumps.
pub type ISJ = SJSize;
