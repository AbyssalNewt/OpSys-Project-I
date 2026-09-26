use crate::io_redir::{Command, io_parse};
use nix::errno::Errno;
use nix::libc::{_exit, STDIN_FILENO, STDOUT_FILENO, close, dup2};
use nix::sys::wait::waitpid;
use nix::unistd::{
    ForkResult::{Child, Parent},
    execv, fork,
};
use std::os::fd::{AsRawFd, IntoRawFd};
use std::{ffi::CString, str::FromStr};

pub fn execute(mut cmds: Vec<Command>) {
    let mut i = 0;
    while i < cmds.len() {
        cmds[i] = io_parse(cmds[i].args.to_vec(), cmds[i].input, cmds[i].output).unwrap();
        i += 1;
    }
    for cmd in &cmds { println!("{:?}, {:?}, {:?}", cmd.args, cmd.input, cmd.output); }

    let mut pid_array: Vec<nix::unistd::Pid> = Vec::new();

    for (i, cmd) in cmds.iter_mut().enumerate() {
        // Vec<String> -> Vec<CString>
        let cstr_args: Vec<CString> = cmd
            .args
            .iter()
            .map(|x| CString::from_str(x).unwrap())
            .collect();

        // TODO: make pipe somehow connect previous command to next command except for first pipe
        // which can be a file input and last pipe which can be a file output

        let pid = unsafe { fork().unwrap() };

        match pid {
            Child => {
                if !cmd.input.is_none() {
                    let in_fd = cmd.input.unwrap();

                    if in_fd < 0 {
                        let err = Errno::last();
                        match err {
                            Errno::ENOENT => println!("File not found"),
                            Errno::EACCES => println!("Permission denied"),
                            _ => println!("Error executing command: {}", err),
                        }
                        unsafe {
                            _exit(1);
                        }
                    }

                    unsafe {
                        dup2(in_fd, STDIN_FILENO);
                        close(in_fd);
                    }
                }
                if !cmd.output.is_none() {
                    let out_fd = cmd.output.unwrap();

                    if out_fd < 0 {
                        let err = Errno::last();
                        match err {
                            Errno::ENOENT => println!("File not found"),
                            Errno::EACCES => println!("Permission denied"),
                            _ => println!("Error executing command: {}", err),
                        }
                        unsafe {
                            _exit(1);
                        }
                    }
                    unsafe {
                        dup2(out_fd, STDOUT_FILENO);
                        close(out_fd);
                    }
                }

                for l in i*2+3..=2*cmds.len() {
                    unsafe{ close(l as i32); }
                }

                execv(cstr_args[0].as_c_str(), &cstr_args).unwrap();
            }

            Parent { child } => {
                if let Some(input) = cmd.input { unsafe {close(input);} }
                if let Some(output) = cmd.output { unsafe {close(output);} }
                pid_array.push(child);
            }
        }
    }

    for pid in pid_array {
        waitpid(pid, None).unwrap();
    }
}
