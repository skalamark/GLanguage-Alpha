// Exception

pub struct Exception {
	filename: String,
	linetext: String,
	column_start: usize,
	column_end: usize,
	lineno: usize,
}

impl Exception {
	pub fn new(
		filename_ref: &String,
		linetext_ref: &String,
		column_start: usize,
		column_end: usize,
		lineno: usize,
	) -> Exception {
		let filename: String = String::from(filename_ref);
		let mut linetext: String = String::from(linetext_ref);
		linetext = linetext.trim_end().to_string();
		linetext = linetext.replace("\t", " ");
		Exception { filename, linetext, column_start, column_end, lineno }
	}

	fn generate_exception_string(
		&mut self,
		error_details: &str,
	) -> String {
		let mut pows: String = String::from("^");
		let mut spaces: String = String::new();
		let mut width_linetext: usize = self.linetext.len();

		self.linetext = self.linetext.trim_start().to_string();
		width_linetext = width_linetext - self.linetext.len();
		let new_column_start: usize = self.column_start - width_linetext;
		let new_column_end: usize = self.column_end - width_linetext;

		for _ in new_column_end..new_column_start {
			pows.push_str("^");
		}

		for _ in 0..new_column_start {
			spaces += " ";
		}

		let mut result: String = String::new();
		result.push_str(
			format!("  File \"{}\", line {}\n", &self.filename, self.lineno + 1).as_str(),
		);
		result.push_str(format!("    {}\n", self.linetext).as_str());
		result.push_str(format!("    {}{}\n", spaces, pows).as_str());
		result.push_str(error_details);
		result
	}

	pub fn illegal_char(&mut self) {
		println!(
			"{}",
			self.generate_exception_string("SyntaxError: Invalid character in identifier")
		);
	}
}
