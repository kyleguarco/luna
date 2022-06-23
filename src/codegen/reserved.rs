use core::str::FromStr;

use alloc::string::String;

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
    Assign,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
    ShiftLeft,
    ShiftRight,
    /// Used by [`Reserved::Goto`] to specify labels.
    DoubleColon,
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
            "=" => Ok(Self::Assign),
            "==" => Ok(Self::Equal),
            ">=" => Ok(Self::GreaterEqual),
            "<=" => Ok(Self::LessEqual),
            "~=" => Ok(Self::NotEqual),
            "<<" => Ok(Self::ShiftLeft),
            ">>" => Ok(Self::ShiftRight),
            "::" => Ok(Self::DoubleColon),
            _ => {
                if let Ok(num) = token.parse::<i32>() {
                    return Ok(Self::Integer(num));
                }

                if let Ok(num) = token.parse::<f32>() {
                    return Ok(Self::Float(num));
                }

                Err(())
            } // todo!("Work on strings!")
        }
    }
}

pub enum ExpressionKind {
    Void,
    Nil,
    True,
    False,
    KConstant,
    KFloat,
    KInteger,
    KString,
    NonReloc,
    Local,
    UpVal,
    Static,
    Index,
    UpIndex,
    IntegerIndex,
    StringIndex,
    Jump,
    Reloc,
    Call,
    VarArg,
}

impl ExpressionKind {
    fn is_indexed(&self) -> bool {
        match self {
            Self::Index | Self::UpIndex | Self::IntegerIndex | Self::StringIndex => true,
            _ => false,
        }
    }

    fn is_variable(&self) -> bool {
        match self {
            Self::Local | Self::UpVal | Self::Static => self.is_indexed(),
            _ => false,
        }
    }
}
