use core::{fmt::Display, ops::Deref, str::FromStr};

use alloc::vec::Vec;

use crate::zio::{Read, Zio};

#[derive(Clone, Debug)]
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
    DoubleColon,
    EndOfScript,
    Float,
    Integer,
    Name,
    String,
}

impl FromStr for Reserved {
    type Err = ();

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        match token {
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
            "//" => Ok(Self::FloorDiv),
            ".." => Ok(Self::Concat),
            "..." => Ok(Self::Dots),
            "==" => Ok(Self::Equal),
            ">=" => Ok(Self::GreaterEqual),
            "<=" => Ok(Self::LessEqual),
            "~=" => Ok(Self::NotEqual),
            "<<" => Ok(Self::ShiftLeft),
            ">>" => Ok(Self::ShiftRight),
            "::" => Ok(Self::DoubleColon),
            "<eof>" => Ok(Self::EndOfScript),
            "<number>" => Ok(Self::Float),
            "<integer>" => Ok(Self::Integer),
            "<name>" => Ok(Self::Name),
            "<string>" => Ok(Self::String),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Token {
    line: usize,
    ttype: Reserved,
}

impl Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.ttype)
    }
}

#[derive(Clone, Debug)]
struct LexerError<'a> {
    token: &'a Token,
    lexeme: &'a str,
    msg: &'static str,
}

impl Display for LexerError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[L{}] Error on \"{}\" (parsed as {}): {}",
            self.token.line, self.lexeme, self.token, self.msg
        )
    }
}

type LexerResult<'a> = Result<(), LexerError<'a>>;

struct Lines<B> {
    buffer: B,
}

struct LexerState {
    source: Zio,
    tokens: Vec<Token>,
    pos: usize,
}

impl LexerState {
    fn new<T: Into<Zio>>(source: T) -> Self {
        LexerState {
            source: source.into(),
            tokens: Vec::new(),
            pos: 0,
        }
    }

    fn scan(&mut self) -> LexerResult {
        let mut buff = [0u8; 32];

        while !self.is_at_end() {
            if let Ok(adv) = self.source.read(&mut buff) {

            } else {
                return Err(LexerError {})
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.stream().len()
    }
}
