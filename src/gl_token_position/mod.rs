// TokenPosition

pub struct TokenPosition {
	pub column: usize,
	pub lineno: usize,
}

impl TokenPosition {
	pub fn copy(&self) -> TokenPosition {
		TokenPosition { column: self.column, lineno: self.lineno }
	}
}
