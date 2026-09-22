use std::ffi::CStr;
use std::ffi::CString;

pub fn execute(path: String, args: Vec<String>) -> Result<usize, nix::Error> {
	let mut keepthecstringsalive: Vec<CString> = Vec::new();

	for arg in args {
		let cstring = CString::new(arg).map_err(|_| nix::errno::Errno::EINVAL)?;
			
		keepthecstringsalive.push(cstring);
	}

	let fixedpathref = CString::new(path).map_err(|_| nix::errno::Errno::EINVAL)?;
	let fixedpath = fixedpathref.as_c_str();

	let mut fixedargs: Vec<&CStr> = Vec::new();
	fixedargs.push(fixedpath);

	for arg in &keepthecstringsalive {
		fixedargs.push(arg.as_c_str());
	}

	let pid = unsafe { nix::unistd::fork()? };

	match pid {
		nix::unistd::ForkResult::Child => {
			// first argument is path
			match nix::unistd::execv(fixedpath, &fixedargs) {
				Ok(_) => std::process::exit(0),
				Err(error) => {
					eprintln!("Forked Child process failed: {}", error);
					std::process::exit(1);
				}
			}			
		}

		nix::unistd::ForkResult::Parent { child } => {
			nix::sys::wait::waitpid(child, None)?;
			return Ok(child.as_raw() as usize);
		}
	}
}
