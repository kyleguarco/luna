//! # Non-recursive Parsers
//! These parsers don't rely on recursion to function.

mod attribute;
mod expression;
mod function;
mod operation;
mod statement;
mod table;

// pub fn chunk(input: In) -> Result<Chunk, ()> {
// 	todo!()
// }

// pub(crate) fn block(input: In) -> IRes<Block> {
// 	map(pair(many0(stat), opt(retstat)), Block::from_parser)(input)
// }

// pub(crate) fn stat(input: In) -> IRes<Statement> {
// 	todo!()
// 	// fn do_block(input: &str) -> IRes<Block> {
// 	// 	delimited(tag(KDO), block, tag(KEND))(input)
// 	// }

// 	// fn if_tree(input: &str) -> IRes<IfTree> {
// 	// 	let (input, initial) = pair(delimited(tag(KIF), exp, tag(KTHEN)), block)(input)?;
// 	// 	let (input, elseifs) = many0(pair(delimited(tag(KELSEIF), exp, tag(KTHEN)), block))(input)?;
// 	// 	let (input, otherwise) = opt(delimited(tag(KELSE), block, tag(KEND)))(input)?;
// 	// 	Ok((input, IfTree { initial, elseifs, otherwise }))
// 	// }

// 	// alt((
// 	// 	value(Statement::End, tchar(SEMICOLON)),
// 	// 	map(pair(list(var), list(exp)), Definition::parse_into::<Statement>),
// 	// 	map(functioncall, Statement::FunctionCall),
// 	// 	map(label, Statement::Label),
// 	// 	value(Statement::Break, tag(KBREAK)),
// 	// 	map(preceded(tag(KGOTO), identifier), Statement::Goto),
// 	// 	map(do_block, |bl| Statement::Do(Box::new(bl))),
// 	// 	map(pair(preceded(tag(KWHILE), exp), do_block), Statement::While),
// 	// 	map(
// 	// 		pair(preceded(tag(KREPEAT), block), preceded(tag(KUNTIL), exp)),
// 	// 		|(bl, ex)| Statement::RepeatUntil(bl, ex),
// 	// 	),
// 	// 	map(if_tree, Statement::IfTree),
// 	// 	map(
// 	// 		tuple((
// 	// 			preceded(tag(KFOR), identifier),
// 	// 			tuple((exp, exp, opt(exp))),
// 	// 			do_block,
// 	// 		)),
// 	// 		|arg| ForExpression::from_parser(arg).into(),
// 	// 	),
// 	// 	map(
// 	// 		pair(
// 	// 			preceded(
// 	// 				tag(KFOR),
// 	// 				separated_pair(list(identifier), tag(KIN), list(exp)),
// 	// 			),
// 	// 			do_block,
// 	// 		),
// 	// 		Statement::ForList,
// 	// 	),
// 	// 	map(
// 	// 		preceded(tag(KFUNCTION), pair(funcname, funcbody)),
// 	// 		Statement::FunctionDefinition,
// 	// 	),
// 	// 	map(
// 	// 		preceded(
// 	// 			tag(KLOCAL),
// 	// 			preceded(tag(KFUNCTION), pair(identifier, funcbody)),
// 	// 		),
// 	// 		Statement::LocalFunctionDefinition,
// 	// 	),
// 	// 	map(
// 	// 		preceded(
// 	// 			tag(KLOCAL),
// 	// 			pair(list(pair(identifier, attrib)), opt(list(exp))),
// 	// 		),
// 	// 		Statement::LocalDefinitionWithAttribute,
// 	// 	),
// 	// ))(input)
// }

// // pub(crate) fn attrib(input: In) -> IRes<Attribute> {
// // 	map(
// // 		opt(delimited(tchar(LESS), identifier, tchar(GREATER))),
// // 		Attribute,
// // 	)(input)
// // }

// // pub(crate) fn retstat(input: In) -> IRes<ReturnStatement> {
// // 	map(
// // 		delimited(tag(KRETURN), opt(list(exp)), tchar(SEMICOLON)),
// // 		ReturnStatement,
// // 	)(input)
// // }

// // pub(crate) fn label(input: In) -> IRes<Label> {
// // 	map(
// // 		delimited(tag(DOUBLECOLON), identifier, tag(DOUBLECOLON)),
// // 		Label,
// // 	)(input)
// // }

// // pub(crate) fn funcname(input: In) -> IRes<FunctionIdentifier> {
// // 	map(
// // 		pair(
// // 			separated_list1(tchar(DOT), identifier),
// // 			opt(preceded(tchar(COLON), identifier)),
// // 		),
// // 		FunctionIdentifier::from_parser,
// // 	)(input)
// // }

// // #[inline(always)]
// // pub(crate) fn var(input: In) -> IRes<Variable> {
// // 	use Variable::*;

// // 	alt((
// // 		map(identifier, Identifier),
// // 		map(pair(prefixexp, bracket(exp)), |(pexp, exp)| {
// // 			PrefixExpressionIndex(Box::new(pexp), Box::new(exp))
// // 		}),
// // 		map(
// // 			separated_pair(prefixexp, tchar(DOT), identifier),
// // 			|(pexp, ident)| PrefixExpressionIdentifier(Box::new(pexp), ident),
// // 		),
// // 	))(input)
// // }

// // pub(crate) fn exp(input: In) -> IRes<Expression> {
// // 	todo!()
// // }

// // pub(crate) fn prefixexp(input: In) -> IRes<PrefixExpression> {
// // 	use PrefixExpression::*;

// // 	alt((
// // 		map(var, Variable),
// // 		map(functioncall, |fcall| FunctionCall(Box::new(fcall))),
// // 		map(paren(exp), ClosedExpression),
// // 	))(input)
// // }

// // pub(crate) fn functioncall(input: In) -> IRes<FunctionCall> {
// // 	todo!()
// // }

// // pub(crate) fn args(input: In) -> IRes<Arguments> {
// // 	todo!()
// // }

// // pub(crate) fn functiondef(input: In) -> IRes<FunctionDefinition> {
// // 	todo!()
// // }

// // pub(crate) fn funcbody(input: In) -> IRes<FunctionBody> {
// // 	todo!()
// // }

// // pub(crate) fn parlist(input: In) -> IRes<ParameterList> {
// // 	use ParameterList::*;

// // 	alt((
// // 		map(
// // 			separated_pair(list(identifier), tchar(COMMA), tag(TRIPLEDOT)),
// // 			|(ilist, _)| IdentifierListWithVarArgs(ilist),
// // 		),
// // 		map(list(identifier), IdentifierList),
// // 		value(VarArgs, tag(TRIPLEDOT)),
// // 	))(input)
// // }

// // pub(crate) fn tableconstructor(input: In) -> IRes<TableConstructor> {
// // 	map(braces(opt(fieldlist)), TableConstructor)(input)
// // }

// // pub(crate) fn fieldlist(input: In) -> IRes<FieldList> {
// // 	terminated(separated_list1(fieldsep, field), opt(fieldsep))(input)
// // }

// // pub(crate) fn field(input: In) -> IRes<Field> {
// // 	use Field::*;

// // 	alt((
// // 		map(assign(bracket(exp), exp), BracketField),
// // 		map(assign(identifier, exp), IdentifierField),
// // 		map(exp, Expression),
// // 	))(input)
// // }

// // pub(crate) fn fieldsep(input: In) -> IRes<In> {
// // 	recognize(alt((tchar(COMMA), tchar(SEMICOLON))))(input)
// // }
