use std::{str::FromStr, ffi::{CString}};
use nix::unistd::{fork, execv, ForkResult::{Child, Parent}};
use nix::sys::wait::waitpid;

pub fn execute(args: Vec<String>) -> Option<usize> {
    // Vec<String> -> Vec<CString>
    let cstr_args: Vec<CString> = args.iter()
        .map(|x| CString::from_str(x).unwrap())
        .collect();

    let pid = unsafe { fork().unwrap() };

    match pid {
        Child => {
            // first argument is path
            execv(cstr_args[0].as_c_str(), &cstr_args).unwrap();
            None
        }

        Parent { child } => {
            waitpid(child, None).unwrap();
            Some(child.as_raw() as usize)
        }
    }
}
