# Traits for certain functions

In Rust, we can implement traits for functions like this:
```rust
trait FnLua {
    fn register(&self) {}
}

impl<T> FnLua for T where T: Fn(i32) -> i32 {}

// `test` is implicitly `FnLua` (notice the signature)
fn test(i: i32) -> i32 { 0 }

fn lib() {
	test.register();
}
``` or
```rust
trait Reg {
	fn register(self);
}

impl<T> Reg for T
where
	T: Fn(i64) -> i32
{
	fn register(self) {
		self(0);
	}
}

macro_rules! register_func {
	( $( $x:expr),* ) => {
		$(
			$x.register();
		)*
	};
}
```
It will likely be easy to register functions in lua like this.
