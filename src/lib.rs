#![no_std]

extern crate alloc;

// The utility libraries
mod mem;
mod zio;
mod gc;
mod debug;

// State and object libraries
mod ltypes;

// Codegen libraries
mod codegen;

// Virtual machine internals
mod lvm;

#[cfg(test)]
mod test {
	const SCRIPT: &'static str = r#"
local a = "hello, world!"

function do_a_thing()
	print(a)
end
"#;

	#[test]
	fn do_something() {

	}
}
