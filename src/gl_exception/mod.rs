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
}
