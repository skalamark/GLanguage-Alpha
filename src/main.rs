// A language of script - version alpha

mod cli;
mod gl_env;
mod gl_exception;
mod gl_interpreter;
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

// Run the interpreter with the code passed from the command line
fn cmd(codetext: &str) {
	let mut interpreter = gl_interpreter::Interpreter::new("cmd");
	interpreter.run_codetext(codetext);
}

// Run the interpreter with the code of a script file
fn run(file: &str) {
	// path to the file
	let path_file: &std::path::Path = std::path::Path::new(file);
	// if it exists and is of the file type
	if path_file.exists() && path_file.is_file() {
		// if the file extension is `.gl`
		if file.ends_with(".gl") {
			let codetext: String = std::fs::read_to_string(&file).expect("");
			let mut interpreter = gl_interpreter::Interpreter::new(file);
			interpreter.run_codetext(codetext.as_str());
		} else {
			println!("GL: Invalid file extension, expected file with extension '.gl'");
		}
	} else {
		println!("GL: Can't open file '{}': No such file", &file);
	}
}
