// Exception

pub struct Exception {
	filename: String,
	linetext: String,
	column_start: usize,
	column_end: usize,
	lineno: usize,
}
