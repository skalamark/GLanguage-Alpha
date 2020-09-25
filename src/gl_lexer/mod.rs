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
