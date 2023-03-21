use crate::{
	combinator::whitespace,
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
	attribute::{Attribute, AttributeName},
	expression::{Expression, ExpressionList, PrefixExpression, ReturnStatement},
	function::{FunctionBody, FunctionCall, FunctionIdentifier},
	operation::{InfixOperation, PrefixOperation},
	statement::{ForExpression, IfTree, Statement},
	variable::{Variable, VariableList},
	AnonFunctionDefinition, Arguments, AttributeNameList, Block, Field, FieldList, IdentifierList,
	Label, ParameterList, TableConstructor,
};
use nom::{
	branch::alt,
	bytes::streaming::tag,
	character::streaming::char,
	combinator::{map, opt, recognize, value},
	multi::{many0, many1, separated_list1},
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
	IResult,
};

pub fn block(input: &str) -> IResult<&str, Block> {
	dbg!(input);
	let (input, (slist, ret)) = whitespace(pair(many0(stat), opt(retstat)))(input)?;
	Ok((input, Block(slist, ret)))
}

pub fn stat(input: &str) -> IResult<&str, Statement> {
	dbg!(input);
	fn do_block(input: &str) -> IResult<&str, Block> {
		dbg!(input);
		delimited(keyword(Keyword::Do), block, keyword(Keyword::End))(input)
	}

	fn if_tree(input: &str) -> IResult<&str, IfTree> {
		dbg!(input);
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
		Ok((input, IfTree { initial, elseifs, otherwise }))
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
			|(ident, (start, stop, step), bl)| {
				ForExpression { ident, start, stop, step, bl }.into()
			},
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
	dbg!(input);
	fn inner(input: &str) -> IResult<&str, AttributeName> {
		dbg!(input);
		let (input, ident) = identifier(input)?;
		let (input, attr) = attrib(input)?;
		Ok((input, AttributeName(ident, attr)))
	}

	let (input, alist) = many1(terminated(inner, tag(COMMA)))(input)?;
	Ok((input, AttributeNameList(alist)))
}

pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	dbg!(input);
	let (input, attr) = opt(delimited(tag(LESS), identifier, tag(GREATER)))(input)?;
	Ok((input, Attribute(attr)))
}

pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	dbg!(input);
	let (input, _) = keyword(Keyword::Return)(input)?;
	let (input, elist) = opt(explist)(input)?;
	let (input, _) = opt(tag(SEMICOLON))(input)?;
	Ok((input, ReturnStatement(elist)))
}

pub fn label(input: &str) -> IResult<&str, Label> {
	dbg!(input);
	let (input, _) = tag(DOUBLECOLON)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, _) = tag(DOUBLECOLON)(input)?;
	Ok((input, Label(ident)))
}

pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	dbg!(input);
	let (input, ilist) = separated_list1(tag(DOT), identifier)(input)?;
	let (input, objident) = opt(preceded(tag(COLON), identifier))(input)?;
	Ok((input, FunctionIdentifier { ilist, objident }))
}

pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	dbg!(input);
	map(separated_list1(tag(COMMA), var), VariableList)(input)
}

pub fn var(input: &str) -> IResult<&str, Variable> {
	dbg!(input);
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
	dbg!(input);
	map(many1(terminated(identifier, tag(COMMA))), IdentifierList)(input)
}

pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	dbg!(input);
	let (input, elist) = many1(terminated(exp, tag(COMMA)))(input)?;
	Ok((input, ExpressionList(elist)))
}

pub fn exp(input: &str) -> IResult<&str, Expression> {
	dbg!(input);
	alt((
		value(Expression::Nil, keyword(Keyword::Nil)),
		value(Expression::False, keyword(Keyword::False)),
		value(Expression::True, keyword(Keyword::True)),
		map(numeral, Expression::Numeral),
		map(literal_string, Expression::LiteralString),
		value(Expression::VarArgs, tag(TRIPLEDOT)),
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
	dbg!(input);
	alt((
		map(var, PrefixExpression::Variable),
		map(functioncall, |fcall| {
			PrefixExpression::FunctionCall(Box::new(fcall))
		}),
		map(
			delimited(tag(LEFTPAREN), exp, tag(RIGHTPAREN)),
			PrefixExpression::ClosedExpression,
		),
	))(input)
}

pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	dbg!(input);
	alt((
		map(pair(prefixexp, args), |(pexp, argu)| {
			FunctionCall::CallFunction(pexp, argu)
		}),
		map(
			pair(separated_pair(prefixexp, tag(COLON), identifier), args),
			|((pexp, ident), argu)| FunctionCall::CallObjectFunction(pexp, ident, argu),
		),
	))(input)
}

pub fn args(input: &str) -> IResult<&str, Arguments> {
	dbg!(input);
	alt((
		map(
			delimited(char('('), opt(explist), tag(RIGHTPAREN)),
			Arguments::ClosedExpressionList,
		),
		map(tableconstructor, Arguments::TableConstructor),
		map(literal_string, Arguments::LiteralString),
	))(input)
}

pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	dbg!(input);
	map(
		preceded(keyword(Keyword::Function), funcbody),
		AnonFunctionDefinition,
	)(input)
}

pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	dbg!(input);
	let (input, plist) = delimited(tag(LEFTPAREN), opt(parlist), tag(RIGHTPAREN))(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(Keyword::End)(input)?;
	Ok((input, FunctionBody(plist, bl)))
}

pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	dbg!(input);
	alt((
		map(
			pair(namelist, opt(recognize(pair(tag(COMMA), tag(TRIPLEDOT))))),
			|(nlist, vargs)| match vargs {
				Some(_) => ParameterList::IdentifierListWithVarArgs(nlist),
				None => ParameterList::IdentifierList(nlist),
			},
		),
		value(ParameterList::VarArgs, recognize(tag(TRIPLEDOT))),
	))(input)
}

pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	dbg!(input);
	map(
		delimited(tag(LEFTBRACE), opt(fieldlist), tag(RIGHTBRACE)),
		|flist| TableConstructor(flist),
	)(input)
}

pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	dbg!(input);
	map(
		terminated(separated_list1(fieldsep, field), opt(fieldsep)),
		FieldList,
	)(input)
}

pub fn field(input: &str) -> IResult<&str, Field> {
	dbg!(input);
	alt((
		map(
			separated_pair(
				delimited(tag(LEFTBRACKET), exp, tag(RIGHTBRACKET)),
				tag(EQUALS),
				exp,
			),
			|(ex1, ex2)| Field::BracketField(ex1, ex2),
		),
		map(
			separated_pair(identifier, tag(EQUALS), exp),
			|(ident, ex)| Field::IdentifierField(ident, ex),
		),
	))(input)
}

pub fn fieldsep(input: &str) -> IResult<&str, &str> {
	dbg!(input);
	alt((tag(COMMA), tag(SEMICOLON)))(input)
}

pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	dbg!(input);
	alt((
		value(InfixOperation::Add, tag(PLUS)),
		value(InfixOperation::Subtract, tag(MINUS)),
		value(InfixOperation::Multiply, tag(STAR)),
		value(InfixOperation::Divide, tag(SLASH)),
		value(InfixOperation::FloorDivide, tag(DOUBLESLASH)),
		value(InfixOperation::Power, tag(CARET)),
		value(InfixOperation::Modulo, tag(PERCENT)),
		value(InfixOperation::BitwiseAnd, tag(AMPHERSAND)),
		value(InfixOperation::BitwiseXor, tag(TILDE)),
		value(InfixOperation::BitwiseOr, tag(PIPE)),
		value(InfixOperation::BitwiseRightShift, tag(SHIFTRIGHT)),
		value(InfixOperation::BitwiseLeftShift, tag(SHIFTLEFT)),
		value(InfixOperation::Concat, tag(DOUBLEDOT)),
		value(InfixOperation::LessThan, tag(LESS)),
		value(InfixOperation::LessEqual, tag(LESSEQUAL)),
		value(InfixOperation::GreaterThan, tag(GREATER)),
		value(InfixOperation::GreaterEqual, tag(GREATEREQUAL)),
		value(InfixOperation::IsEqual, tag(ISEQUAL)),
		value(InfixOperation::IsNotEqual, tag(NOTEQUAL)),
		value(InfixOperation::And, keyword(Keyword::And)),
		value(InfixOperation::Or, keyword(Keyword::Or)),
	))(input)
}

pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	dbg!(input);
	alt((
		value(PrefixOperation::Not, tag(MINUS)),
		value(PrefixOperation::Negate, keyword(Keyword::Not)),
		value(PrefixOperation::Length, tag(OCTOTHORPE)),
		value(PrefixOperation::BitwiseNot, tag(TILDE)),
	))(input)
}
