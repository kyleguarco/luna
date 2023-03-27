use luna_ast::types::{Identifier, LiteralString, Numeral};
use nom::{
	branch::alt,
	bytes::complete::{is_not, tag, take_while_m_n},
	character::complete::{alpha1, alphanumeric1, char, digit1, multispace1},
	combinator::{map, map_opt, map_res, recognize, value, verify},
	error::{FromExternalError, ParseError},
	multi::{fold_many0, many0_count},
	sequence::{delimited, pair, preceded},
	IResult,
};

pub(crate) mod keyword;
pub(crate) mod string;

// https://github.com/rust-bakery/nom/blob/main/examples/string.rs

/// Parses identifiers (which share naming conventions with Rust).
/// Lua refers to identifiers as "Names" in their documentation.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#rust-style-identifiers
pub fn identifier(input: &str) -> IResult<&str, Identifier> {
	dbg!(input);
	let (input, result) = recognize(pair(
		alt((alpha1, tag("_"))),
		many0_count(alt((alphanumeric1, tag("_")))),
	))(input)?;

	Ok((input, Identifier(String::from(result))))
}

/// Parses integers and floats.
///
/// Examples of valid integers: `3` `345` `0xff` `0xBEBADA`
/// <br>
/// Examples of valid floats: `3.0` `3.1416` `314.16e-2`
/// `0.31416E1` `34e1` `0x0.1E` `0xA23p-4` `0X1.921FB54442D18P+1`
pub fn numeral(input: &str) -> IResult<&str, Numeral> {
	// TODO: Support floats and hex digits
	map(map_res(digit1, str::parse::<isize>), Numeral::Integer)(input)
}

/// Parse a unicode sequence, of the form u{XXXX}, where XXXX is 1 to 6
/// hexadecimal numerals. We will combine this later with parse_escaped_char
/// to parse sequences like \u{00AC}.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
fn parse_unicode<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
	E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
	// `take_while_m_n` parses between `m` and `n` bytes (inclusive) that match
	// a predicate. `parse_hex` here parses between 1 and 6 hexadecimal numerals.
	let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

	// `preceded` takes a prefix parser, and if it succeeds, returns the result
	// of the body parser. In this case, it parses u{XXXX}.
	let parse_delimited_hex = preceded(
		char('u'),
		// `delimited` is like `preceded`, but it parses both a prefix and a suffix.
		// It returns the result of the middle parser. In this case, it parses
		// {XXXX}, where XXXX is 1 to 6 hex numerals, and returns XXXX
		delimited(char('{'), parse_hex, char('}')),
	);

	// `map_res` takes the result of a parser and applies a function that returns
	// a Result. In this case we take the hex bytes from parse_hex and attempt to
	// convert them to a u32.
	let parse_u32 = map_res(parse_delimited_hex, move |hex| u32::from_str_radix(hex, 16));

	// map_opt is like map_res, but it takes an Option instead of a Result. If
	// the function returns None, map_opt returns an error. In this case, because
	// not all u32 values are valid unicode code points, we have to fallibly
	// convert to char with from_u32.
	map_opt(parse_u32, std::char::from_u32)(input)
}

/// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
	E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
	preceded(
		char('\\'),
		// `alt` tries each parser in sequence, returning the result of
		// the first successful match
		alt((
			parse_unicode,
			// The `value` parser returns a fixed value (the first argument) if its
			// parser (the second argument) succeeds. In these cases, it looks for
			// the marker characters (n, r, t, etc) and returns the matching
			// character (\n, \r, \t, etc).
			value('\n', char('n')),
			value('\r', char('r')),
			value('\t', char('t')),
			value('\u{08}', char('b')),
			value('\u{0C}', char('f')),
			value('\\', char('\\')),
			value('/', char('/')),
			value('"', char('"')),
		)),
	)(input)
}

/// Parse a backslash, followed by any amount of whitespace. This is used later
/// to discard any escaped whitespace.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
fn parse_escaped_whitespace<'a, E: ParseError<&'a str>>(
	input: &'a str,
) -> IResult<&'a str, &'a str, E> {
	preceded(char('\\'), multispace1)(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
	// `is_not` parses a string of 0 or more characters that aren't one of the
	// given characters.
	let not_quote_slash = is_not("\"\\");

	// `verify` runs a parser, then runs a verification function on the output of
	// the parser. The verification function accepts out output only if it
	// returns true. In this case, we want to ensure that the output of is_not
	// is non-empty.
	verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

/// A string fragment contains a fragment of a string being parsed: either
/// a non-empty Literal (a series of non-escaped characters), a single
/// parsed escaped character, or a block of escaped whitespace.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
	Literal(&'a str),
	EscapedChar(char),
	EscapedWS,
}

/// Combine parse_literal, parse_escaped_whitespace, and parse_escaped_char
/// into a StringFragment.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
	E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
	alt((
		// The `map` combinator runs a parser, then applies a function to the output
		// of that parser.
		map(parse_literal, StringFragment::Literal),
		map(parse_escaped_char, StringFragment::EscapedChar),
		value(StringFragment::EscapedWS, parse_escaped_whitespace),
	))(input)
}

/// Parse a string. Use a loop of parse_fragment and push all of the fragments
/// into an output string.
/// into a StringFragment.
///
/// Copied from [the nom string parsing examples][2]
///
/// [2]: https://github.com/rust-bakery/nom/blob/1c1e9c896bd422baa8ae0a167be57ae721f10377/examples/string.rs
pub fn literal_string<'a, E>(input: &'a str) -> IResult<&'a str, LiteralString, E>
where
	E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
	// TODO: Support multi-line strings

	// fold is the equivalent of iterator::fold. It runs a parser in a loop,
	// and for each output value, calls a folding function on each output value.
	let build_string = fold_many0(
		// Our parser function– parses a single string fragment
		parse_fragment,
		// Our init value, an empty string
		String::new,
		// Our folding function. For each fragment, append the fragment to the
		// string.
		|mut string, fragment| {
			match fragment {
				StringFragment::Literal(s) => string.push_str(s),
				StringFragment::EscapedChar(c) => string.push(c),
				StringFragment::EscapedWS => {}
			}
			string
		},
	);

	// Finally, parse the string. Note that, if `build_string` could accept a raw
	// " character, the closing delimiter " would never match. When using
	// `delimited` with a looping parser (like fold), be sure that the
	// loop won't accidentally match your closing delimiter!
	map(delimited(char('"'), build_string, char('"')), LiteralString)(input)
}
