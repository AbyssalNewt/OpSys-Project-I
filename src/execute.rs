use std::{str::FromStr, ffi::{CString}};
use nix::errno::Errno;
use nix::libc::{STDIN_FILENO, dup2, close, STDOUT_FILENO, _exit};
use nix::unistd::{fork, execv, ForkResult::{Child, Parent}};
use nix::sys::wait::waitpid;
use crate::io_redir::Command;

pub fn execute(cmd: Command) {
    // Vec<String> -> Vec<CString>
    let cstr_args: Vec<CString> = cmd.args.iter()
        .map(|x| CString::from_str(x).unwrap())
        .collect();

    let pid = unsafe { fork().unwrap() };

    match pid {
        Child => {
            if !cmd.input.is_none()
            {
                let in_fd = cmd.input.unwrap();

                if in_fd < 0 {
                    let err = Errno::last();
                    match err{
                        Errno::ENOENT => println!("File not found"),
                        Errno::EACCES => println!("Permission denied"),
                        _ => println!("Error executing command: {}", err),
                    }
                    unsafe { _exit(1); }
                }

                unsafe {
                    dup2(in_fd, STDIN_FILENO);
                    close(in_fd);
                }
            }
            if !cmd.output.is_none()
            {
                let out_fd = cmd.output.unwrap();

                if out_fd < 0 {
                    let err = Errno::last();
                    match err{
                        Errno::EACCES => println!("Permission denied"),
                        _ => println!("Error executing command: {}", err),
                    }
                    unsafe { _exit(1); }
                }
                unsafe {
                    dup2(out_fd, STDOUT_FILENO);
                    close(out_fd);
                }
            }

            // first argument is path
            execv(cstr_args[0].as_c_str(), &cstr_args).unwrap();

        }

        Parent { child } => {
            waitpid(child, None).unwrap();

        }
    }
}
