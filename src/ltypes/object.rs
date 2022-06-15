/// Collectible garbage types
pub trait Garbage {
    fn next<T: Garbage>() -> T;
}

pub struct TValue {}

pub struct TString {}

pub struct UValue {}

pub struct UData {}

pub struct UDataNoUser {}

pub struct UpValDescription {}

pub struct LocalVar {}

pub struct AbsLineInfo {}

/// Function prototype
pub struct Proto {}

pub struct UpVal {}

pub struct NativeClosure {}

pub struct LuaClosure {}

pub enum Closure {
    Native(NativeClosure),
    Lua(LuaClosure),
}

pub struct Table {}
