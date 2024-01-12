macro_rules! keyword {
	($($name:ident = $val:literal),*) => {
		$(pub const $name: &str = $val;)*
	};
}

keyword! {
	KAND = "and", KBREAK = "break", KDO = "do",
	KELSE = "else", KELSEIF = "elseif", KEND = "end",
	KFALSE = "false", KFOR = "for", KFUNCTION = "function",
	KGOTO = "goto", KIF = "if", KIN = "in",
	KLOCAL = "local", KNIL = "nil", KNOT = "not",
	KOR = "or", KREPEAT = "repeat", KRETURN = "return",
	KTHEN = "then", KTRUE = "true", KUNTIL = "until",
	KWHILE = "while"
}

pub fn is_keyword(input: &str) -> bool {
	match input {
		KAND | KBREAK | KDO | KELSE | KELSEIF | KEND | KFALSE | KFOR | KFUNCTION | KGOTO | KIF
		| KIN | KLOCAL | KNIL | KNOT | KOR | KREPEAT | KRETURN | KTHEN | KTRUE | KUNTIL
		| KWHILE => true,
		_ => false,
	}
}
