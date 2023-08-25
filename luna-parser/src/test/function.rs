use crate::parse::function::functioncall;

#[test]
fn call() {
	println!("{:?}", functioncall("fun()"));
}
