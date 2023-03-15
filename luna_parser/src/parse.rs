use crate::{
	combinator::whitetag,
	terminal::{
		identifier,
		keyword::{keyword, Keyword},
		literal_string, numeral,
		string::{
			AMPHERSAND, CARET, COLON, COMMA, DOT, DOUBLECOLON, DOUBLEDOT, DOUBLESLASH, EQUALS,
			GREATER, GREATEREQUAL, ISEQUAL, LEFTBRACE, LEFTBRACKET, LEFTPAREN, LESS, LESSEQUAL,
			MINUS, NOTEQUAL, OCTOTHORPE, PERCENT, PIPE, PLUS, RIGHTBRACE, RIGHTBRACKET, RIGHTPAREN,
			SEMICOLON, SHIFTLEFT, SHIFTRIGHT, SLASH, STAR, TILDE, TRIPLEDOT,
		},
	},
};
use luna_ast::types::{
	AnonFunctionDefinition, Arguments, Attribute, AttributeName, AttributeNameList, Block,
	Expression, ExpressionList, Field, FieldList, FunctionBody, FunctionCall, FunctionIdentifier,
	IdentifierList, IfTree, InfixOperation, Label, ParameterList, PrefixExpression,
	PrefixOperation, ReturnStatement, Statement, TableConstructor, Variable, VariableList,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, opt, recognize, value},
	multi::{many0, many1, separated_list1},
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
	IResult,
};

pub fn block(input: &str) -> IResult<&str, Block> {
	let (input, slist) = many0(stat)(input)?;
	let (input, ret) = opt(retstat)(input)?;
	Ok((input, Block(slist, ret)))
}

pub fn stat(input: &str) -> IResult<&str, Statement> {
	fn do_block(input: &str) -> IResult<&str, Block> {
		delimited(keyword(Keyword::Do), block, keyword(Keyword::End))(input)
	}

	fn if_tree(input: &str) -> IResult<&str, IfTree> {
		let (input, initial) = pair(
			delimited(keyword(Keyword::If), exp, keyword(Keyword::Then)),
			block,
		)(input)?;
		let (input, elseifs) = many0(pair(
			delimited(keyword(Keyword::ElseIf), exp, keyword(Keyword::Then)),
			block,
		))(input)?;
		let (input, otherwise) = opt(delimited(
			keyword(Keyword::Else),
			block,
			keyword(Keyword::End),
		))(input)?;
		Ok((
			input,
			IfTree {
				initial,
				elseifs,
				otherwise,
			},
		))
	}

	alt((
		value(Statement::End, tag(SEMICOLON)),
		// This comes before 'varlist' to detect the presence of 'local'
		map(
			preceded(keyword(Keyword::Local), pair(attnamelist, opt(explist))),
			Statement::LocalDefinitionWithAttribute,
		),
		map(pair(varlist, explist), Statement::Definition),
		map(functioncall, Statement::FunctionCall),
		map(label, Statement::Label),
		value(Statement::Break, keyword(Keyword::Break)),
		map(
			preceded(keyword(Keyword::Goto), identifier),
			Statement::Goto,
		),
		map(do_block, |bl| Statement::Do(Box::new(bl))),
		map(
			pair(preceded(keyword(Keyword::While), exp), do_block),
			Statement::While,
		),
		map(
			pair(
				preceded(keyword(Keyword::Repeat), block),
				preceded(keyword(Keyword::Until), exp),
			),
			|(bl, ex)| Statement::RepeatUntil(bl, ex),
		),
		map(if_tree, Statement::IfTree),
		map(
			tuple((
				preceded(keyword(Keyword::For), identifier),
				tuple((exp, exp, opt(exp))),
				do_block,
			)),
			Statement::ForExpression,
		),
		map(
			pair(
				preceded(
					keyword(Keyword::For),
					separated_pair(namelist, keyword(Keyword::In), explist),
				),
				do_block,
			),
			Statement::ForList,
		),
		map(
			preceded(keyword(Keyword::Function), pair(funcname, funcbody)),
			Statement::FunctionDefinition,
		),
		map(
			preceded(
				keyword(Keyword::Local),
				preceded(keyword(Keyword::Function), pair(identifier, funcbody)),
			),
			Statement::LocalFunctionDefinition,
		),
	))(input)
}

pub fn attnamelist(input: &str) -> IResult<&str, AttributeNameList> {
	fn inner(input: &str) -> IResult<&str, AttributeName> {
		let (input, ident) = identifier(input)?;
		let (input, attr) = attrib(input)?;
		Ok((input, AttributeName(ident, attr)))
	}

	let (input, alist) = many1(terminated(inner, whitetag(COMMA)))(input)?;
	Ok((input, AttributeNameList(alist)))
}

pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	let (input, attr) = opt(delimited(whitetag(LESS), identifier, whitetag(GREATER)))(input)?;
	Ok((input, Attribute(attr)))
}

pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	let (input, _) = keyword(Keyword::Return)(input)?;
	let (input, elist) = opt(explist)(input)?;
	let (input, _) = opt(whitetag(SEMICOLON))(input)?;
	Ok((input, ReturnStatement(elist)))
}

pub fn label(input: &str) -> IResult<&str, Label> {
	let (input, _) = whitetag(DOUBLECOLON)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, _) = whitetag(DOUBLECOLON)(input)?;

	Ok((input, Label(ident)))
}

pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	let (input, ilist) = separated_list1(whitetag(DOT), identifier)(input)?;
	let (input, objident) = opt(preceded(whitetag(COLON), identifier))(input)?;
	Ok((input, FunctionIdentifier { ilist, objident }))
}

pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	let (input, vlist) = separated_list1(whitetag(COMMA), var)(input)?;
	Ok((input, VariableList(vlist)))
}

pub fn var(input: &str) -> IResult<&str, Variable> {
	alt((
		map(identifier, Variable::Identifier),
		map(
			pair(
				prefixexp,
				delimited(tag(LEFTBRACKET), exp, tag(RIGHTBRACKET)),
			),
			|(pexp, exp)| Variable::PrefixExpressionIndex(Box::new(pexp), Box::new(exp)),
		),
		map(
			separated_pair(prefixexp, tag(DOT), identifier),
			|(pexp, ident)| Variable::PrefixExpressionIdentifier(Box::new(pexp), ident),
		),
	))(input)
}

pub fn namelist(input: &str) -> IResult<&str, IdentifierList> {
	let (input, ilist) = many1(terminated(identifier, whitetag(COMMA)))(input)?;
	Ok((input, IdentifierList(ilist)))
}

pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	let (input, elist) = many1(terminated(exp, whitetag(COMMA)))(input)?;
	Ok((input, ExpressionList(elist)))
}

pub fn exp(input: &str) -> IResult<&str, Expression> {
	alt((
		value(Expression::Nil, keyword(Keyword::Nil)),
		value(Expression::False, keyword(Keyword::False)),
		value(Expression::True, keyword(Keyword::True)),
		map(numeral, Expression::Numeral),
		map(literal_string, Expression::LiteralString),
		value(Expression::VarArgs, whitetag(TRIPLEDOT)),
		map(functiondef, Expression::AnonFunctionDefinition),
		map(prefixexp, |pexp| {
			Expression::PrefixExpression(Box::new(pexp))
		}),
		map(tableconstructor, Expression::TableConstructor),
		map(tuple((exp, binop, exp)), |(expa, itype, expb)| {
			Expression::InfixOperation(Box::new(expa), itype, Box::new(expb))
		}),
		map(pair(unop, exp), |(utype, exp)| {
			Expression::PrefixOperation(utype, Box::new(exp))
		}),
	))(input)
}

pub fn prefixexp(input: &str) -> IResult<&str, PrefixExpression> {
	alt((
		map(var, |v| PrefixExpression::Variable(v)),
		map(functioncall, |fcall| {
			PrefixExpression::FunctionCall(Box::new(fcall))
		}),
		map(
			delimited(whitetag(LEFTPAREN), exp, whitetag(RIGHTPAREN)),
			|ex| PrefixExpression::ClosedExpression(ex),
		),
	))(input)
}

pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	alt((
		map(pair(prefixexp, args), |(pexp, argu)| {
			FunctionCall::CallFunction(pexp, argu)
		}),
		map(
			pair(separated_pair(prefixexp, whitetag(COLON), identifier), args),
			|((pexp, ident), argu)| FunctionCall::CallObjectFunction(pexp, ident, argu),
		),
	))(input)
}

pub fn args(input: &str) -> IResult<&str, Arguments> {
	alt((
		map(
			delimited(whitetag(LEFTPAREN), opt(explist), whitetag(RIGHTPAREN)),
			|elist| Arguments::ClosedExpressionList(elist),
		),
		map(tableconstructor, |tbc| Arguments::TableConstructor(tbc)),
		map(literal_string, |ls| Arguments::LiteralString(ls)),
	))(input)
}

pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	map(preceded(keyword(Keyword::Function), funcbody), |fbody| {
		AnonFunctionDefinition(fbody)
	})(input)
}

pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	let (input, plist) = delimited(whitetag(LEFTPAREN), opt(parlist), whitetag(RIGHTPAREN))(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(Keyword::End)(input)?;
	Ok((input, FunctionBody(plist, bl)))
}

pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	alt((
		map(
			pair(
				namelist,
				opt(recognize(pair(whitetag(COMMA), whitetag(TRIPLEDOT)))),
			),
			|(nlist, vargs)| match vargs {
				Some(_) => ParameterList::IdentifierListWithVarArgs(nlist),
				None => ParameterList::IdentifierList(nlist),
			},
		),
		value(ParameterList::VarArgs, recognize(whitetag(TRIPLEDOT))),
	))(input)
}

pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	map(
		delimited(whitetag(LEFTBRACE), opt(fieldlist), whitetag(RIGHTBRACE)),
		|flist| TableConstructor(flist),
	)(input)
}

pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	map(
		terminated(separated_list1(fieldsep, field), opt(fieldsep)),
		|flist| FieldList(flist),
	)(input)
}

pub fn field(input: &str) -> IResult<&str, Field> {
	alt((
		map(
			separated_pair(
				delimited(whitetag(LEFTBRACKET), exp, whitetag(RIGHTBRACKET)),
				whitetag(EQUALS),
				exp,
			),
			|(ex1, ex2)| Field::BracketField(ex1, ex2),
		),
		map(
			separated_pair(identifier, whitetag(EQUALS), exp),
			|(ident, ex)| Field::IdentifierField(ident, ex),
		),
	))(input)
}

pub fn fieldsep(input: &str) -> IResult<&str, &str> {
	alt((whitetag(COMMA), whitetag(SEMICOLON)))(input)
}

pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	alt((
		value(InfixOperation::Add, whitetag(PLUS)),
		value(InfixOperation::Subtract, whitetag(MINUS)),
		value(InfixOperation::Multiply, whitetag(STAR)),
		value(InfixOperation::Divide, whitetag(SLASH)),
		value(InfixOperation::FloorDivide, whitetag(DOUBLESLASH)),
		value(InfixOperation::Power, whitetag(CARET)),
		value(InfixOperation::Modulo, whitetag(PERCENT)),
		value(InfixOperation::BitwiseAnd, whitetag(AMPHERSAND)),
		value(InfixOperation::BitwiseXor, whitetag(TILDE)),
		value(InfixOperation::BitwiseOr, whitetag(PIPE)),
		value(InfixOperation::BitwiseRightShift, whitetag(SHIFTRIGHT)),
		value(InfixOperation::BitwiseLeftShift, whitetag(SHIFTLEFT)),
		value(InfixOperation::Concat, whitetag(DOUBLEDOT)),
		value(InfixOperation::LessThan, whitetag(LESS)),
		value(InfixOperation::LessEqual, whitetag(LESSEQUAL)),
		value(InfixOperation::GreaterThan, whitetag(GREATER)),
		value(InfixOperation::GreaterEqual, whitetag(GREATEREQUAL)),
		value(InfixOperation::IsEqual, whitetag(ISEQUAL)),
		value(InfixOperation::IsNotEqual, whitetag(NOTEQUAL)),
		value(InfixOperation::And, keyword(Keyword::And)),
		value(InfixOperation::Or, keyword(Keyword::Or)),
	))(input)
}

pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	alt((
		value(PrefixOperation::Not, whitetag(MINUS)),
		value(PrefixOperation::Negate, keyword(Keyword::Not)),
		value(PrefixOperation::Length, whitetag(OCTOTHORPE)),
		value(PrefixOperation::BitwiseNot, whitetag(TILDE)),
	))(input)
}
