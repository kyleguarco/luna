use crate::terminal::keyword::{keyword, Keyword};

#[test]
fn test_keywords() {
	let input = r#"
	and
	break
	do
	else
	elseif
	end
	false
	for
	function
	goto
	if
	in
	local
	nil
	not
	or
	repeat
	return
	then
	true
	until
	while
	"#;

	fn check(key: Keyword, sl: &str) -> &str {
		let lit = key.literal();
		let val = keyword::<&str, nom::error::Error<&str>>(key)(sl);
		assert!(val.is_ok());
		let val = val.unwrap();
		assert_eq!(val.1, lit);

		val.0
	}

	let input = check(Keyword::And, input);
	let input = check(Keyword::Break, input);
	let input = check(Keyword::Do, input);
	let input = check(Keyword::Else, input);
	let input = check(Keyword::ElseIf, input);
	let input = check(Keyword::End, input);
	let input = check(Keyword::False, input);
	let input = check(Keyword::For, input);
	let input = check(Keyword::Function, input);
	let input = check(Keyword::Goto, input);
	let input = check(Keyword::If, input);
	let input = check(Keyword::In, input);
	let input = check(Keyword::Local, input);
	let input = check(Keyword::Nil, input);
	let input = check(Keyword::Not, input);
	let input = check(Keyword::Or, input);
	let input = check(Keyword::Repeat, input);
	let input = check(Keyword::Return, input);
	let input = check(Keyword::Then, input);
	let input = check(Keyword::True, input);
	let input = check(Keyword::Until, input);
	let input = check(Keyword::While, input);

	assert!(input.is_empty());
}
