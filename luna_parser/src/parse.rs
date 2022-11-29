use crate::{
    cursor::Cursor,
    error::{ParseError, ParseErrorType},
    lex::Lexer,
    token::{Token, TokenType},
};

pub type ParseResult = Result<(), ParseError>;

struct Parser<'src> {
    lexer: Lexer<'src>,
    look: Option<Token<'src>>,
}

impl<'src> TryFrom<Lexer<'src>> for Parser<'src> {
    type Error = ParseError;

    fn try_from(mut lexer: Lexer<'src>) -> Result<Self, Self::Error> {
        let look = lexer
            .next()
            .ok_or_else(|| ParseError::new(ParseErrorType::NothingToParse, Cursor::default()))?
            .map_err(|err| ParseError::new(ParseErrorType::InvalidToken, err.cursor))?;
        Ok(Self {
            lexer,
            look: Some(look),
        })
    }
}

impl<'src> Parser<'src> {
    pub fn parse(mut self) {
        self.chunk();
    }

    fn accept(&mut self, ttype: TokenType) -> ParseResult {
        match &self.look {
            Some(token) => match token.token_type() == &ttype {
                true => {
                    self.look = self.advance();
                    Ok(())
                }
                false => Err(ParseError::new(
                    ParseErrorType::InvalidToken,
                    *token.cursor(),
                )),
            },
            None => Err(ParseError::new(
                ParseErrorType::NothingToParse,
                Cursor::default(),
            )),
        }
    }

    fn advance(&mut self) -> Option<Token<'src>> {
        self.lexer.next().transpose().ok().flatten()
    }

    fn next_ttype(&self) -> Option<&TokenType> {
        match &self.look {
            Some(token) => Some(token.token_type()),
            None => None,
        }
    }

    fn chunk(&mut self) {
        self.block();
    }

    fn block(&mut self) {
        self.stat();
        self.retstat();
    }

    fn stat(&mut self) -> ParseResult {
        match self.next_ttype() {
            Some(TokenType::Semicolon) => self.accept(TokenType::Semicolon),
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn attnamelist(&mut self) -> ParseResult {
        self.accept(TokenType::Identifier)?;
        self.attrib()?;
        loop {
            match self.next_ttype() {
                Some(TokenType::Comma) => self.attnamelist()?,
                _ => break Ok(())
            }
        }
    }

    fn attrib(&mut self) -> ParseResult {
        match self.next_ttype() {
            Some(TokenType::LessThan) => {
                self.accept(TokenType::LessThan)?;
                self.accept(TokenType::Identifier)?;
                self.accept(TokenType::GreaterThan)
            },
            _ => Ok(())
        }
    }

    fn retstat(&mut self) -> ParseResult {
        todo!()
    }

    fn label(&mut self) -> ParseResult {
        todo!()
    }

    fn funcname(&mut self) -> ParseResult {
        todo!()
    }

    fn varlist(&mut self) -> ParseResult {
        todo!()
    }

    fn var(&mut self) -> ParseResult {
        todo!()
    }

    fn namelist(&mut self) -> ParseResult {
        todo!()
    }

    fn explist(&mut self) -> ParseResult {
        todo!()
    }

    fn exp(&mut self) -> ParseResult {
        todo!()
    }

    fn prefixexp(&mut self) -> ParseResult {
        todo!()
    }

    fn functioncall(&mut self) -> ParseResult {
        todo!()
    }

    fn args(&mut self) -> ParseResult {
        todo!()
    }

    fn functiondef(&mut self) -> ParseResult {
        todo!()
    }

    fn funcbody(&mut self) -> ParseResult {
        todo!()
    }

    fn parlist(&mut self) -> ParseResult {
        todo!()
    }

    fn tableconstructor(&mut self) -> ParseResult {
        todo!()
    }

    fn fieldlist(&mut self) -> ParseResult {
        todo!()
    }

    fn field(&mut self) -> ParseResult {
        todo!()
    }

    fn fieldsep(&mut self) -> ParseResult {
        todo!()
    }

    fn binop(&mut self) -> ParseResult {
        todo!()
    }

    fn unop(&mut self) -> ParseResult {
        todo!()
    }

}
