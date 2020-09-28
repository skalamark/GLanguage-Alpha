// Token

use crate::gl_env::Env;
use crate::gl_exception::Exception;
use crate::gl_token_position::TokenPosition;
use crate::gl_tokens::Tokens;

pub struct Token {
	pub typer: Tokens,
	pub linetext: String,
	pub position_start: TokenPosition,
	pub position_end: TokenPosition,
}

impl Token {
	pub fn illegal_char(
		&self,
		env: &Env,
	) {
		let mut exception: Exception = Exception::new(
			&env.filename,
			&self.linetext,
			self.position_start.column,
			self.position_end.column,
			env.lineno,
		);
		exception.illegal_char();
	}
}
