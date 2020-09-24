// Cli

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

// Analyzes the arguments passed by the command line
pub fn parse_args(version: &str) -> ArgMatches<'static> {
	App::new("GLanguage")
		.settings(&[AppSettings::DisableHelpSubcommand])
		.version(format!("v{}", version).as_str())
		.usage("gl [options] [arguments]\n    gl run main.gl [arguments]")
		.help_message("Prints this help message and exit.")
		.version_message("Prints version info and exit.")
		.subcommand(
			SubCommand::with_name("cmd")
				.about("Run program passed as string")
				.display_order(0)
				.arg(Arg::with_name("codetext").empty_values(true).required(true)),
		)
		.subcommand(
			SubCommand::with_name("run")
				.about("Run program from a script file")
				.display_order(1)
				.arg(Arg::with_name("file").empty_values(false).required(true)),
		)
		.get_matches()
}
