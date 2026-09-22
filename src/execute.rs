use std::ffi::CStr;
use std::ffi::CString;

pub fn execute(path: Vec<String>) -> Result<usize, nix::Error> {
	let mut keepthecstringsalive: Vec<CString> = Vec::new();

	for arg in path {
		let cstring = CString::new(arg).map_err(|_| nix::errno::Errno::EINVAL)?;
			
		keepthecstringsalive.push(cstring);
	}

	let fixedpathref = keepthecstringsalive[0].clone();
	let fixedpath = fixedpathref.as_c_str();

	keepthecstringsalive.drain(..2);

	let mut fixedargs: Vec<&CStr> = Vec::new();

	for arg in &keepthecstringsalive {
		fixedargs.push(arg.as_c_str());
	}

	let pid = unsafe { nix::unistd::fork()? };

	match pid {
		nix::unistd::ForkResult::Child => {
			// this may be wrong idk how to make the arguments not include the path and im too tired to figure out rn
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
