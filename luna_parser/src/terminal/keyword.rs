pub const KAND: &str = "and";
pub const KBREAK: &str = "break";
pub const KDO: &str = "do";
pub const KELSE: &str = "else";
pub const KELSEIF: &str = "elseif";
pub const KEND: &str = "end";
pub const KFALSE: &str = "false";
pub const KFOR: &str = "for";
pub const KFUNCTION: &str = "function";
pub const KGOTO: &str = "goto";
pub const KIF: &str = "if";
pub const KIN: &str = "in";
pub const KLOCAL: &str = "local";
pub const KNIL: &str = "nil";
pub const KNOT: &str = "not";
pub const KOR: &str = "or";
pub const KREPEAT: &str = "repeat";
pub const KRETURN: &str = "return";
pub const KTHEN: &str = "then";
pub const KTRUE: &str = "true";
pub const KUNTIL: &str = "until";
pub const KWHILE: &str = "while";

pub fn is_keyword(input: &str) -> bool {
	match input {
		KAND | KBREAK | KDO | KELSE | KELSEIF | KEND | KFALSE | KFOR | KFUNCTION | KGOTO | KIF
		| KIN | KLOCAL | KNIL | KNOT | KOR | KREPEAT | KRETURN | KTHEN | KTRUE | KUNTIL
		| KWHILE => true,
		_ => false,
	}
}
