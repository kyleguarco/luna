use crate::terminal::{
	identifier,
	keyword::{keyword, Keyword},
};

fn check_keyword(key: Keyword, sl: &str) -> &str {
	let lit = key.literal();
	let val = keyword::<&str, nom::error::Error<&str>>(key)(sl).unwrap();
	assert_eq!(val.1, lit);

	val.0
}

#[test]
fn test_identifiers() {
	let input = r#"
	if name then
		something
	end
	"#;

	let input = check_keyword(Keyword::If, input);
	let (input, name) = identifier(input).unwrap();
	assert_eq!(name.0, "name");
	let input = check_keyword(Keyword::Then, input);
	let (input, name) = identifier(input).unwrap();
	assert_eq!(name.0, "something");
	let input = check_keyword(Keyword::End, input);

	assert!(input.is_empty());
}

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

	let input = check_keyword(Keyword::And, input);
	let input = check_keyword(Keyword::Break, input);
	let input = check_keyword(Keyword::Do, input);
	let input = check_keyword(Keyword::Else, input);
	let input = check_keyword(Keyword::ElseIf, input);
	let input = check_keyword(Keyword::End, input);
	let input = check_keyword(Keyword::False, input);
	let input = check_keyword(Keyword::For, input);
	let input = check_keyword(Keyword::Function, input);
	let input = check_keyword(Keyword::Goto, input);
	let input = check_keyword(Keyword::If, input);
	let input = check_keyword(Keyword::In, input);
	let input = check_keyword(Keyword::Local, input);
	let input = check_keyword(Keyword::Nil, input);
	let input = check_keyword(Keyword::Not, input);
	let input = check_keyword(Keyword::Or, input);
	let input = check_keyword(Keyword::Repeat, input);
	let input = check_keyword(Keyword::Return, input);
	let input = check_keyword(Keyword::Then, input);
	let input = check_keyword(Keyword::True, input);
	let input = check_keyword(Keyword::Until, input);
	let input = check_keyword(Keyword::While, input);

	assert!(input.is_empty());
}
