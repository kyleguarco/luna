use crate::terminal::{keyword::{kand, kgoto}, identifier};

#[test]
fn test_keywords() {
	let input = "and goto Name";

	dbg!(input);
	let (input, k) = kand(input).unwrap();
	dbg!(k);
	let (input, _) = kgoto(input).unwrap();
	let (input, id) = identifier(input).unwrap();
	dbg!((input, id));
	assert!(input.is_empty());
}
