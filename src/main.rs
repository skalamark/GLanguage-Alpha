// A language of script - version alpha

mod cli;

const GL_VERSION: &str = "0.1.0-alpha"; // Current version

// Function main
fn main() {
	let matches = cli::parse_args(GL_VERSION);

	// Check which command was called
	match matches.subcommand() {
		_ => (),
	}
}
