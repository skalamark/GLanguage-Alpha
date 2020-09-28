// Lexer

use crate::gl_env::Env;
use crate::gl_token::Token;
use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::{Tokens, SPACES};

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

	fn build_new_token(
		&mut self,
		typer: Tokens,
		pos_start: TokenPosition,
	) -> Token {
		Token {
			typer,
			linetext: String::from(&self.current_linetext),
			position_start: pos_start,
			position_end: self.position.copy(),
		}
	}

	fn add_build_new_token(
		&mut self,
		typer: Tokens,
		pos_start: TokenPosition,
	) {
		let token = self.build_new_token(typer, pos_start);
		self.tokens.push(token);
	}

	fn illegal_char(
		&mut self,
		env: &mut Env,
	) {
		let pos_start: TokenPosition = self.position.copy();
		let token: Token = self.build_new_token(Tokens::EOF, pos_start);
		token.illegal_char(env);
		self.advance_linetext(env);
	}

	fn make_next_token(
		&mut self,
		env: &mut Env,
	) -> bool {
		let pos_start: TokenPosition = self.position.copy();

		if self.current_char.is_empty() {
			if self.tokens.len() > 0 {
				self.add_build_new_token(
					Tokens::EOF,
					self.tokens[self.tokens.len() - 1].position_end.copy(),
				);
			} else {
				self.add_build_new_token(Tokens::EOF, pos_start);
			}
			return false;
		} else if SPACES.contains(&self.current_char.as_str()) {
			if self.current_char == "\n" {
				self.advance_linetext(env);
				self.advance_char();
			} else {
				self.advance();
			}
		} else {
			self.illegal_char(env);
			return true;
		}

		self.make_next_token(env)
	}
}
