#[derive(Debug)]
pub struct LexError<'src> {
    pub lexeme: &'src str,
    pub line: usize,
    pub col: usize,
}

impl<'src> LexError<'src> {
    pub(crate) fn new(lexeme: &'src str, line: usize, col: usize) -> Self {
        Self {
            lexeme,
            line,
            col,
        }
    }
}
