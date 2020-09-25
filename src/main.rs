// A language of script - version alpha

mod cli;
mod gl_env;
mod gl_exception;
mod gl_lexer;
mod gl_token;
mod gl_token_position;
mod gl_tokens;

const GL_VERSION: &str = "0.1.0-alpha"; // Current version

// Function main
fn main() {
	let matches = cli::parse_args(GL_VERSION);

	// Check which command was called
	match matches.subcommand() {
		_ => (),
	}
}
