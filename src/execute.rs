pub fn execute(path: Vec<String>) => Result<usize, Error> {
	unsafe {
		let pid = nix::unistd::fork();
		if(pid == 0) {
			nix::unistd::execv(path);
		} else {
			nix::unistd::wait(pid);
		}
		Ok()
	}
}
