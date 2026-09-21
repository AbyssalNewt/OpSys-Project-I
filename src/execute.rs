use std::ffi::CStr;
use std::ffi::CString;

pub fn execute(path: Vec<String>) -> Result<usize, nix::Error> {
	let fixedpath: Vec<core::ffi::CStr>;
	for arg in path {
		fixedpath.push(CStr::new(arg.as_str().unwrap()).as_c_str());
	}
	let pid = nix::unistd::fork();
	if pid.unwrap() == 0 {
		nix::unistd::execv(fixedpath[0],fixedpath[1..]);
	} else {
		nix::sys::wait::waitpid(pid.unwrap());
	}
	Ok()
}
