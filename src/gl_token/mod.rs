// Token

use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::Tokens;

pub struct Token {
	pub typer: Tokens,
	pub linetext: String,
	pub position_start: TokenPosition,
	pub position_end: TokenPosition,
}
