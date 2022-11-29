#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub(crate) fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

	pub(crate) fn newline(&mut self) {
		self.line += 1;
		self.column = 0;
	}

    pub(crate) fn advance(&mut self) {
        self.column += 1;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new(1, 0)
    }
}
