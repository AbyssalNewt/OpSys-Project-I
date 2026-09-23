use std::{str::FromStr, ffi::{CString}};
use nix::unistd::{fork, execv, ForkResult::{Child, Parent}};
use nix::sys::wait::waitpid;

pub fn execute(args: Vec<String>) -> Result<usize, nix::Error> {
    // Vec<String> -> Vec<CString>
    let cstr_args: Vec<CString> = args.iter()
        .map(|x| CString::from_str(x).unwrap())
        .collect();

    let pid = unsafe { fork()? };

    match pid {
        Child => {
            // first argument is path
            match execv(cstr_args[0].as_c_str(), &cstr_args[1..]) {
                Ok(_) => std::process::exit(0),
                Err(error) => {
                    eprintln!("Forked Child process failed: {}", error);
                    std::process::exit(1);
                }
            }			
        }

        Parent { child } => {
            waitpid(child, None)?;
            return Ok(child.as_raw() as usize);
        }
    }
}
