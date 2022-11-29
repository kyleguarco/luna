use luna_types::num::{LuaNumber, LuaInteger};

use crate::{cursor::Cursor, error::LexErrorType};

#[rustfmt::skip]
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
	// Keywords
	And, Break, Do, Else, ElseIf, End, False, For, Function,
	Goto, If, In, Local, Nil, Not, Or, Repeat, Return, Then,
	True, Until, While,
	// Terminal Symbols (single)
	Plus, Minus, Star, Slash, Percent, Caret, Pound, Amphersand,
	Tilde, Pipe, LessThan, GreaterThan, Equal, LeftParen,
	RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
	Dot, Colon, Comma, Semicolon,
	// Terminal Symbols (greater than one)
	ShiftLeft, ShiftRight, DoubleSlash, Equality, NotEqual,
	LessEqual, GreaterEqual, DoubleColon, DoubleDots, TripleDots,
    // Literals
    Identifier, String, Integer(LuaInteger), Float(LuaNumber),
    /// The nothing type (for parsing errors)
    Nothing
}

impl TokenType {
    pub fn is_terminal(c: &char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '%' | '^' | '#' | '&' | '~' | '|' | '<' | '>' | '=' | '('
            | ')' | '{' | '}' | '[' | ']' | '.' | ':' | ',' | ';' => true,
            _ => false,
        }
    }

    pub fn try_keyword(s: &str) -> Result<Self, LexErrorType> {
        match s {
            "and" => Ok(Self::And),
            "break" => Ok(Self::Break),
            "do" => Ok(Self::Do),
            "else" => Ok(Self::Else),
            "elseif" => Ok(Self::ElseIf),
            "end" => Ok(Self::End),
            "false" => Ok(Self::False),
            "for" => Ok(Self::For),
            "function" => Ok(Self::Function),
            "goto" => Ok(Self::Goto),
            "if" => Ok(Self::If),
            "in" => Ok(Self::In),
            "local" => Ok(Self::Local),
            "nil" => Ok(Self::Nil),
            "not" => Ok(Self::Not),
            "or" => Ok(Self::Or),
            "repeat" => Ok(Self::Repeat),
            "return" => Ok(Self::Return),
            "then" => Ok(Self::Then),
            "true" => Ok(Self::True),
            "until" => Ok(Self::Until),
            "while" => Ok(Self::While),
            _ => Err(LexErrorType::InvalidKeyword),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token<'src> {
    ttype: TokenType,
    lexeme: &'src str,
    cursor: Cursor,
}

impl<'src> Token<'src> {
    pub(crate) fn new(ttype: TokenType, lexeme: &'src str, cursor: Cursor) -> Self {
        Self {
            ttype,
            lexeme,
            cursor,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.ttype
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn lexeme(&self) -> &'src str {
        self.lexeme
    }
}
