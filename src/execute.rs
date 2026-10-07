use {
    crate::io_redir::{Command, io_parse},
    nix::{
        errno::Errno,
        libc::{_exit, STDIN_FILENO, STDOUT_FILENO, close, dup2},
        sys::wait::waitpid,
        unistd::{
            ForkResult::{Child, Parent},
            Pid, execv, fork,
        },
    },
    std::{ffi::CString, str::FromStr},
};

/// execute - Runs array of commands with plag for background processing.
/// 
/// - cmds : array of commands which include the args and the input/output fds for use on the child
/// - bg_flag : true for background processing and false for no background processing
pub fn execute(mut cmds: Vec<Command>, bg_flag: bool) -> Result<Pid,String> {
    // first pass through commands to check for input/output redirection
    let mut i = 0;
    while i < cmds.len() {
        let cmd = match io_parse(cmds[i].args.to_vec(), cmds[i].input, cmds[i].output)
        {
            Ok(cmd) => cmd,
            Err(cmd) => return Err(cmd),
        };
        cmds[i] = cmd;
        i += 1;
    }

    // need to store array of child process ids for waiting and management
    let mut pid_array: Vec<nix::unistd::Pid> = Vec::new();

    // main execution loop, forks then applies any stream replacement then executes command in child
    for (i, cmd) in cmds.iter_mut().enumerate() {
        // Vec<String> -> Vec<CString>
        let cstr_args: Vec<CString> = cmd
            .args
            .iter()
            .map(|x| CString::from_str(x).unwrap())
            .collect();

        let pid = unsafe { fork().unwrap() };

        match pid {
            // child process
            Child => {
                if let Some(in_fd) = cmd.input {
                    // check for file descriptor to replace stdin fd
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
                if let Some(out_fd) = cmd.output {
                    // check for file descriptor to replace stdout fd
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

                // close all of the other file descriptors in the array since they are duplicated in
                // child process
                for l in i * 2 + 3..=2 * cmds.len() {
                    unsafe {
                        close(l as i32);
                    }
                }
                
                // replace current program with the command intended for execution
                execv(cstr_args[0].as_c_str(), &cstr_args).unwrap();
            }

            // parent process
            Parent { child } => {
                // close command input fd on parent
                if let Some(input) = cmd.input {
                    unsafe {
                        close(input);
                    }
                }
                // close command output fd on parent
                if let Some(output) = cmd.output {
                    unsafe {
                        close(output);
                    }
                }
                // push new forked child pid to array of pids
                pid_array.push(child);
            }
        }
    }
    // background processing
    if !bg_flag {
        // if not backgrund processing wait on parent side for every child to finish
        for pid in &pid_array {
            waitpid(*pid, None).unwrap();
        }
    }
    // return last child pid for management purposes and to ensure it finished properly
    Ok(*pid_array.last().unwrap())
}
