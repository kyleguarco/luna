use core::{fmt::Display, ops::Deref, str::FromStr};

use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Reserved {
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
    FloorDiv,
    Concat,
    Dots,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
    ShiftLeft,
    ShiftRight,
    /// Used by [`Reserved::Goto`] to specify labels.
    DoubleColon,
    /// Allows for squishing [`Reserved::Name`]`s betwixt quotes.
    StringBound,
    EndOfScript,
    Float(f32),
    Integer(i32),
    Name(String),
    String(String),
}

// Why not FromStr or From<&str>? It's because both types have
// requirements that don't serve this enum. FromStr has the potential
// to fail; From<&str> doesn't fail, but requires that Into<Reserved>
// be satisfied, which isn't guaranteed with a string slice. Any
// errors in parsing will be caught by the code generator, not the lexer.
impl Reserved {
    fn parse(token: &str) -> Self {
        match token {
            "and" => Self::And,
            "break" => Self::Break,
            "do" => Self::Do,
            "else" => Self::Else,
            "elseif" => Self::ElseIf,
            "end" => Self::End,
            "false" => Self::False,
            "for" => Self::For,
            "function" => Self::Function,
            "goto" => Self::Goto,
            "if" => Self::If,
            "in" => Self::In,
            "local" => Self::Local,
            "nil" => Self::Nil,
            "not" => Self::Not,
            "or" => Self::Or,
            "repeat" => Self::Repeat,
            "return" => Self::Return,
            "then" => Self::Then,
            "true" => Self::True,
            "until" => Self::Until,
            "while" => Self::While,
            "//" => Self::FloorDiv,
            ".." => Self::Concat,
            "..." => Self::Dots,
            "==" => Self::Equal,
            ">=" => Self::GreaterEqual,
            "<=" => Self::LessEqual,
            "~=" => Self::NotEqual,
            "<<" => Self::ShiftLeft,
            ">>" => Self::ShiftRight,
            "::" => Self::DoubleColon,
            "\"" => Self::StringBound,
            _ => {
                if let Ok(num) = token.parse::<i32>() {
                    return Self::Integer(num);
                }

                if let Ok(num) = token.parse::<f32>() {
                    return Self::Float(num);
                }

                Self::Name(token.to_owned())
            }
            // todo!("Work on strings!")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    ttype: Reserved,
    line: usize,
}

impl Token {
    fn new(ttype: Reserved, line: usize) -> Self {
        Token { ttype, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.ttype)
    }
}

pub struct LexerState<'a> {
    source: &'a str,
}

impl<'a> LexerState<'a> {
    pub fn new<T: Deref<Target = str>>(source: &'a T) -> Self {
        LexerState { source }
    }

    pub fn scan(self) -> Vec<Token> {
        let mut tokens = Vec::new();

        for (lnum, line) in self.source.lines().enumerate() {
            tokens.extend(
                line.split(" ")
                    .map(|lexeme| Reserved::parse(lexeme))
                    .map(|ttype| Token::new(ttype, lnum)),
            );
        }

        tokens
    }
}
