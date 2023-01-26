use nom::IResult;

mod stat;

fn chunk(input: &str) -> IResult<&str, ()> {
	block(input)
}

fn block(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn stat(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn attnamelist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn attrib(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn retstat(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn label(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn funcname(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn varlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn var(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn namelist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn explist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn exp(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn prefixexp(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn functioncall(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn args(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn functiondef(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn funcbody(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn parlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn tableconstructor(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn fieldlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn field(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn fieldsep(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn binop(input: &str) -> IResult<&str, ()> {
	todo!()
}

fn unop(input: &str) -> IResult<&str, ()> {
	todo!()
}
