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

	pub fn run_codetext(
		&mut self,
		codetext: &str,
	) -> bool {
		let mut lexer: Lexer = Lexer::new(codetext, &self.env);
		if lexer.run(&mut self.env) {
			return true;
		}
		false
	}
}
