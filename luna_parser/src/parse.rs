//! # Non-recursive Parsers
//! These parsers don't rely on recursion to function.

mod attribute;
mod expression;
mod function;
mod misc;
mod operation;
mod statement;
mod table;
mod variable;

// pub fn chunk(input: In) -> Result<Chunk, ()> {
// 	todo!()
// }

// pub(crate) fn block(input: In) -> IRes<Block> {
// 	map(pair(many0(stat), opt(retstat)), Block::from_parser)(input)
// }

// pub(crate) fn stat(input: In) -> IRes<Statement> {
// 	todo!()
// 	fn do_block(input: &str) -> IRes<Block> {
// 		delimited(tag(KDO), block, tag(KEND))(input)
// 	}

// 	fn if_tree(input: &str) -> IRes<IfTree> {
// 		let (input, initial) = pair(delimited(tag(KIF), exp, tag(KTHEN)), block)(input)?;
// 		let (input, elseifs) = many0(pair(delimited(tag(KELSEIF), exp, tag(KTHEN)), block))(input)?;
// 		let (input, otherwise) = opt(delimited(tag(KELSE), block, tag(KEND)))(input)?;
// 		Ok((input, IfTree { initial, elseifs, otherwise }))
// 	}

// 	alt((
// 		value(Statement::End, tchar(SEMICOLON)),
// 		map(pair(list(var), list(exp)), Definition::parse_into::<Statement>),
// 		map(functioncall, Statement::FunctionCall),
// 		map(label, Statement::Label),
// 		value(Statement::Break, tag(KBREAK)),
// 		map(preceded(tag(KGOTO), name), Statement::Goto),
// 		map(do_block, |bl| Statement::Do(Box::new(bl))),
// 		map(pair(preceded(tag(KWHILE), exp), do_block), Statement::While),
// 		map(
// 			pair(preceded(tag(KREPEAT), block), preceded(tag(KUNTIL), exp)),
// 			|(bl, ex)| Statement::RepeatUntil(bl, ex),
// 		),
// 		map(if_tree, Statement::IfTree),
// 		map(
// 			tuple((
// 				preceded(tag(KFOR), name),
// 				tuple((exp, exp, opt(exp))),
// 				do_block,
// 			)),
// 			|arg| ForExpression::from_parser(arg).into(),
// 		),
// 		map(
// 			pair(
// 				preceded(
// 					tag(KFOR),
// 					separated_pair(list(name), tag(KIN), list(exp)),
// 				),
// 				do_block,
// 			),
// 			Statement::ForList,
// 		),
// 		map(
// 			preceded(tag(KFUNCTION), pair(funcname, funcbody)),
// 			Statement::FunctionDefinition,
// 		),
// 		map(
// 			preceded(
// 				tag(KLOCAL),
// 				preceded(tag(KFUNCTION), pair(name, funcbody)),
// 			),
// 			Statement::LocalFunctionDefinition,
// 		),
// 		map(
// 			preceded(
// 				tag(KLOCAL),
// 				pair(list(pair(name, attrib)), opt(list(exp))),
// 			),
// 			Statement::LocalDefinitionWithAttribute,
// 		),
// 	))(input)
// }

// pub(crate) fn attrib(input: In) -> IRes<Attribute> {
// 	map(
// 		opt(delimited(tchar(LESS), name, tchar(GREATER))),
// 		Attribute,
// 	)(input)
// }

// pub(crate) fn retstat(input: In) -> IRes<ReturnStatement> {
// 	map(
// 		delimited(tag(KRETURN), opt(list(exp)), tchar(SEMICOLON)),
// 		ReturnStatement,
// 	)(input)
// }

// pub(crate) fn label(input: In) -> IRes<Label> {
// 	map(
// 		delimited(tag(DOUBLECOLON), name, tag(DOUBLECOLON)),
// 		Label,
// 	)(input)
// }

// pub(crate) fn funcname(input: In) -> IRes<FunctionName> {
// 	map(
// 		pair(
// 			separated_list1(tchar(DOT), name),
// 			opt(preceded(tchar(COLON), name)),
// 		),
// 		FunctionName::from_parser,
// 	)(input)
// }

// #[inline(always)]
// pub(crate) fn var(input: In) -> IRes<Variable> {
// 	use Variable::*;

// 	alt((
// 		map(name, Name),
// 		map(pair(prefixexp, bracket(exp)), |(pexp, exp)| {
// 			PrefixExpressionIndex(Box::new(pexp), Box::new(exp))
// 		}),
// 		map(
// 			separated_pair(prefixexp, tchar(DOT), name),
// 			|(pexp, name)| PrefixExpressionName(Box::new(pexp), name),
// 		),
// 	))(input)
// }

// pub(crate) fn prefixexp(input: In) -> IRes<PrefixExpression> {
// 	use PrefixExpression::*;

// 	alt((
// 		map(var, Variable),
// 		map(functioncall, |fcall| FunctionCall(Box::new(fcall))),
// 		map(paren(exp), ClosedExpression),
// 	))(input)
// }

// pub(crate) fn functioncall(input: In) -> IRes<FunctionCall> {
// 	todo!()
// }

// pub(crate) fn args(input: In) -> IRes<Arguments> {
// 	todo!()
// }

// pub(crate) fn functiondef(input: In) -> IRes<FunctionDefinition> {
// 	todo!()
// }

// pub(crate) fn funcbody(input: In) -> IRes<FunctionBody> {
// 	todo!()
// }

// pub(crate) fn parlist(input: In) -> IRes<ParameterList> {
// 	use ParameterList::*;

// 	alt((
// 		map(
// 			separated_pair(list(name), tchar(COMMA), tag(TRIPLEDOT)),
// 			|(ilist, _)| NameListWithVarArgs(ilist),
// 		),
// 		map(list(name), NameList),
// 		value(VarArgs, tag(TRIPLEDOT)),
// 	))(input)
// }
