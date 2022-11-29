use crate::cursor::Cursor;

#[derive(Debug)]
pub enum LexErrorType {
    UnknownSequence,
    InvalidKeyword,
    InvalidTerminal,
    InvalidLiteral,
}

#[derive(Debug)]
pub struct LexError<'src> {
    pub etype: LexErrorType,
    pub lexeme: &'src str,
    pub cursor: Cursor,
}

impl<'src> LexError<'src> {
    pub(crate) fn new(etype: LexErrorType, lexeme: &'src str, cursor: Cursor) -> Self {
        Self {
            etype,
            lexeme,
            cursor,
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorType {
    NothingToParse,
    InvalidToken,
}

#[derive(Debug)]
pub struct ParseError {
    pub etype: ParseErrorType,
    pub cursor: Cursor,
}

impl ParseError {
    pub(crate) fn new(etype: ParseErrorType, cursor: Cursor) -> Self {
        Self { etype, cursor }
    }
}
