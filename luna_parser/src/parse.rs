use crate::terminal::{
	identifier,
	keyword::{keyword, Keyword},
	literal_string, numeral,
	string::{
		AMPH, CARET, COLON, COMMA, DOT, DOUBLECOLON, DOUBLEDOT, DOUBLESLASH, EQUALS, GREATER,
		GREATEREQUAL, ISEQUAL, LBRACE, LBRACKET, LESS, LESSEQUAL, LPAREN, LSHIFT, MINUS, NOTEQUAL,
		PERCENT, PIPE, PLUS, POUND, RBRACE, RBRACKET, RPAREN, RSHIFT, SEMICOLON, SLASH, STAR,
		TILDE, TRIPLEDOT,
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
	bytes::complete::tag,
	character::complete::char,
	combinator::{map, opt, value},
	multi::{many0, separated_list1},
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
	IResult,
};

/// Grammar: `block ::= {stat} [retstat]`
pub fn block(input: &str) -> IResult<&str, Block> {
	dbg!(input);
	map(pair(many0(stat), opt(retstat)), |(st, rstat)| {
		Block(st, rstat)
	})(input)
}

/// Grammar:
/// ```!
/// stat ::=  ‘;’ |
///	varlist ‘=’ explist |
///	functioncall |
///	label |
///	break |
///	goto Name |
///	do block end |
///	while exp do block end |
///	repeat block until exp |
///	if exp then block {elseif exp then block} [else block] end |
///	for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
///	for namelist in explist do block end |
///	function funcname funcbody |
///	local function Name funcbody |
///	local attnamelist [‘=’ explist]
/// ```
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
		value(Statement::End, char(SEMICOLON)),
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
		map(
			preceded(keyword(Keyword::Local), pair(attnamelist, opt(explist))),
			Statement::LocalDefinitionWithAttribute,
		),
	))(input)
}

/// Grammar: `attnamelist ::=  Name attrib {‘,’ Name attrib}`
pub fn attnamelist(input: &str) -> IResult<&str, AttributeNameList> {
	dbg!(input);
	fn inner(input: &str) -> IResult<&str, AttributeName> {
		dbg!(input);
		let (input, ident) = identifier(input)?;
		let (input, attr) = attrib(input)?;
		Ok((input, AttributeName(ident, attr)))
	}

	map(separated_list1(char(COMMA), inner), AttributeNameList)(input)
}

/// Grammar: `attrib ::= [‘<’ Name ‘>’]`
pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	dbg!(input);
	let (input, attr) = opt(delimited(char(LESS), identifier, char(GREATER)))(input)?;
	Ok((input, Attribute(attr)))
}

/// Grammar: `retstat ::= return [explist] [‘;’]`
pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	dbg!(input);
	map(
		preceded(
			keyword(Keyword::Return),
			terminated(opt(explist), char(SEMICOLON)),
		),
		ReturnStatement,
	)(input)
}

/// Grammar: `label ::= ‘::’ Name ‘::’`
pub fn label(input: &str) -> IResult<&str, Label> {
	dbg!(input);
	let (input, _) = tag(DOUBLECOLON)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, _) = tag(DOUBLECOLON)(input)?;
	Ok((input, Label(ident)))
}

/// Grammar: `funcname ::= Name {‘.’ Name} [‘:’ Name]`
pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	dbg!(input);
	let (input, ilist) = separated_list1(char(DOT), identifier)(input)?;
	let (input, objident) = opt(preceded(char(COLON), identifier))(input)?;
	Ok((input, FunctionIdentifier { ilist, objident }))
}

/// Grammar: `varlist ::= var {‘,’ var}`
pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	dbg!(input);

	/// Grammar: `var ::= Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name `
	fn var(input: &str) -> IResult<&str, Variable> {
		dbg!(input);
		alt((
			map(identifier, Variable::Identifier),
			map(
				pair(prefixexp, delimited(char(LBRACKET), exp, char(RBRACKET))),
				|(pexp, exp)| Variable::PrefixExpressionIndex(Box::new(pexp), Box::new(exp)),
			),
			map(
				separated_pair(prefixexp, char(DOT), identifier),
				|(pexp, ident)| Variable::PrefixExpressionIdentifier(Box::new(pexp), ident),
			),
		))(input)
	}

	map(separated_list1(char(COMMA), var), VariableList)(input)
}

/// Grammar: `namelist ::= Name {‘,’ Name}`
pub fn namelist(input: &str) -> IResult<&str, IdentifierList> {
	dbg!(input);
	map(separated_list1(char(COMMA), identifier), IdentifierList)(input)
}

/// Grammar: `explist ::= exp {‘,’ exp}`
pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	dbg!(input);
	map(separated_list1(char(COMMA), exp), ExpressionList)(input)
}

/// Grammar:
/// ```!
/// exp ::= nil | false | true | Numeral | LiteralString | ‘...’ |
/// functiondef | prefixexp | tableconstructor | exp binop exp | unop exp
/// ```
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

/// Grammar: `prefixexp ::= var | functioncall | ‘(’ exp ‘)’`
pub fn prefixexp(input: &str) -> IResult<&str, PrefixExpression> {
	dbg!(input);

	/// Grammar: `var ::= Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name `
	fn var(input: &str) -> IResult<&str, Variable> {
		dbg!(input);
		alt((
			map(identifier, Variable::Identifier),
			map(
				pair(prefixexp, delimited(char(LBRACKET), exp, char(RBRACKET))),
				|(pexp, exp)| Variable::PrefixExpressionIndex(Box::new(pexp), Box::new(exp)),
			),
			map(
				separated_pair(prefixexp, char(DOT), identifier),
				|(pexp, ident)| Variable::PrefixExpressionIdentifier(Box::new(pexp), ident),
			),
		))(input)
	}

	alt((
		map(var, PrefixExpression::Variable),
		map(functioncall, |fcall| {
			PrefixExpression::FunctionCall(Box::new(fcall))
		}),
		map(
			delimited(char(LPAREN), exp, char(RPAREN)),
			PrefixExpression::ClosedExpression,
		),
	))(input)
}

/// Grammar: `functioncall ::=  prefixexp args | prefixexp ‘:’ Name args `
pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	dbg!(input);
	alt((
		map(pair(prefixexp, args), |(pexp, argu)| {
			FunctionCall::CallFunction(pexp, argu)
		}),
		map(
			pair(separated_pair(prefixexp, char(COLON), identifier), args),
			|((pexp, ident), argu)| FunctionCall::CallObjectFunction(pexp, ident, argu),
		),
	))(input)
}

/// Grammar: `args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString `
pub fn args(input: &str) -> IResult<&str, Arguments> {
	dbg!(input);
	alt((
		map(
			delimited(char('('), opt(explist), char(RPAREN)),
			Arguments::ClosedExpressionList,
		),
		map(tableconstructor, Arguments::TableConstructor),
		map(literal_string, Arguments::LiteralString),
	))(input)
}

/// Grammar: `functiondef ::= function funcbody`
pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	dbg!(input);
	map(
		preceded(keyword(Keyword::Function), funcbody),
		AnonFunctionDefinition,
	)(input)
}

/// Grammar: `funcbody ::= ‘(’ [parlist] ‘)’ block end`
pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	dbg!(input);
	let (input, plist) = delimited(char(LPAREN), opt(parlist), char(RPAREN))(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(Keyword::End)(input)?;
	Ok((input, FunctionBody(plist, bl)))
}

/// Grammar: `parlist ::= namelist [‘,’ ‘...’] | ‘...’`
pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	dbg!(input);
	alt((
		value(ParameterList::VarArgs, tag(TRIPLEDOT)),
		map(
			pair(namelist, opt(pair(char(COMMA), tag(TRIPLEDOT)))),
			|(nlist, vargs)| match vargs {
				Some(_) => ParameterList::IdentifierListWithVarArgs(nlist),
				None => ParameterList::IdentifierList(nlist),
			},
		),
	))(input)
}

/// Grammar: `tableconstructor ::= ‘{’ [fieldlist] ‘}’`
pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	dbg!(input);
	map(
		delimited(char(LBRACE), opt(fieldlist), char(RBRACE)),
		|flist| TableConstructor(flist),
	)(input)
}

/// Grammar: `fieldlist ::= field {fieldsep field} [fieldsep]`
pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	dbg!(input);
	map(
		terminated(separated_list1(fieldsep, field), opt(fieldsep)),
		FieldList,
	)(input)
}

/// Grammar: `field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp`
pub fn field(input: &str) -> IResult<&str, Field> {
	dbg!(input);
	alt((
		map(
			separated_pair(
				delimited(char(LBRACKET), exp, char(RBRACKET)),
				char(EQUALS),
				exp,
			),
			|(ex1, ex2)| Field::BracketField(ex1, ex2),
		),
		map(
			separated_pair(identifier, char(EQUALS), exp),
			|(ident, ex)| Field::IdentifierField(ident, ex),
		),
	))(input)
}

/// Grammar: `fieldsep ::= ‘,’ | ‘;’`
pub fn fieldsep(input: &str) -> IResult<&str, char> {
	dbg!(input);
	alt((char(COMMA), char(SEMICOLON)))(input)
}

/// Grammar:
/// ```!
/// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
///	‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
///	‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
///	and | or
/// ```
pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	dbg!(input);
	alt((
		value(InfixOperation::Add, char(PLUS)),
		value(InfixOperation::Subtract, char(MINUS)),
		value(InfixOperation::Multiply, char(STAR)),
		value(InfixOperation::Divide, char(SLASH)),
		value(InfixOperation::FloorDivide, tag(DOUBLESLASH)),
		value(InfixOperation::Power, char(CARET)),
		value(InfixOperation::Modulo, char(PERCENT)),
		value(InfixOperation::BitwiseAnd, char(AMPH)),
		value(InfixOperation::BitwiseXor, char(TILDE)),
		value(InfixOperation::BitwiseOr, char(PIPE)),
		value(InfixOperation::BitwiseRightShift, tag(RSHIFT)),
		value(InfixOperation::BitwiseLeftShift, tag(LSHIFT)),
		value(InfixOperation::Concat, tag(DOUBLEDOT)),
		value(InfixOperation::LessThan, char(LESS)),
		value(InfixOperation::LessEqual, tag(LESSEQUAL)),
		value(InfixOperation::GreaterThan, char(GREATER)),
		value(InfixOperation::GreaterEqual, tag(GREATEREQUAL)),
		value(InfixOperation::IsEqual, tag(ISEQUAL)),
		value(InfixOperation::IsNotEqual, tag(NOTEQUAL)),
		value(InfixOperation::And, keyword(Keyword::And)),
		value(InfixOperation::Or, keyword(Keyword::Or)),
	))(input)
}

/// Grammar: `unop ::= ‘-’ | not | ‘#’ | ‘~’`
pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	dbg!(input);
	alt((
		value(PrefixOperation::Not, char(MINUS)),
		value(PrefixOperation::Negate, keyword(Keyword::Not)),
		value(PrefixOperation::Length, char(POUND)),
		value(PrefixOperation::BitwiseNot, char(TILDE)),
	))(input)
}
