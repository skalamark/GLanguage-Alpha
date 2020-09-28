// Interpreter

use crate::gl_env::Env;
use crate::gl_lexer::Lexer;

pub struct Interpreter {
	env: Env,
}

impl Interpreter {
	pub fn new(filename: &str) -> Interpreter {
		let env: Env = Env { filename: filename.to_string(), lineno: 0 };
		Interpreter { env }
	}
}
