// Lexer

use crate::gl_env::Env;
use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;

pub struct Lexer {
	chars: Vec<char>,
	linestext: Vec<String>,
	current_char: String,
	current_linetext: String,
	pub position: TokenPosition,
	pub tokens: Vec<Token>,
}

impl Lexer {
	pub fn new(
		codetext: &str,
		env: &Env,
	) -> Lexer {
		// all chars from codetext
		let mut chars: Vec<char> = codetext.chars().collect();
		// all lines from codetext
		let mut linestext: Vec<String> = codetext.lines().map(|line| line.to_string()).collect();
		// current char
		let current_char: String =
			if chars.len() > 0 { chars.remove(0).to_string() } else { String::new() };
		// current linetext
		let current_linetext: String =
			if linestext.len() > 0 { linestext.remove(0) } else { String::from(codetext) };
		// the position of current char
		let position: TokenPosition = TokenPosition { column: 0, lineno: env.lineno };
		let tokens: Vec<Token> = Vec::new();
		Lexer { chars, linestext, current_char, current_linetext, position, tokens }
	}

	fn advance_char(&mut self) {
		if self.chars.len() > 0 {
			self.current_char = self.chars.remove(0).to_string();
		} else {
			self.current_char = String::new();
		}
	}

	fn advance_linetext(
		&mut self,
		env: &mut Env,
	) {
		env.lineno += 1;
		self.position.column = 0;
		if self.linestext.len() > 0 {
			self.current_linetext = self.linestext.remove(0);
		}
	}

	fn advance(&mut self) {
		self.position.column += 1;
		self.advance_char();
	}
}
