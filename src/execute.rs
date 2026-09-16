pub fn execute(path: Vec<String>) => Result<usize, Error> {
	unsafe {
		execv(path)
	}
}
