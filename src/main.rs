mod env_expansion;
mod execute;
mod io_redir;
mod path_search;
mod prompt;
mod tilde_expansion;

use {
    nix::{
        errno::Errno,
        libc::{c_int, chdir, getenv, setenv},
        sys::wait::{WaitPidFlag, WaitStatus, waitpid},
        unistd::Pid,
    },
    std::{
        env,
        ffi::{CStr, CString},
        os::fd::IntoRawFd,
        str::FromStr,
    },
};

#[derive(Clone, Debug)]
pub struct Job {
    num: i32,
    pid: Pid,
    cmd: String,
}

#[derive(Clone)]
pub struct JobTracker {
    last_job: i32,
    jobs: Vec<Job>,
}
impl JobTracker {
    fn push(&mut self, pid: Pid, cmd: String) {
        self.last_job += 1;
        self.jobs.push(Job {
            num: self.last_job,
            pid,
            cmd,
        });
    }
}

fn main() {
    let mut job_tracker = JobTracker {
        last_job: 0,
        jobs: Vec::new(),
    };

    let mut command_history: Vec<String> = Vec::new();
    let mut bg_flag: bool = false;

    loop {
        // Check for done jobs
        while let Ok(wait_status) = waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            // No jobs done
            if wait_status == WaitStatus::StillAlive {
                break;
            }
            // Find index of done job. Uses ifLet because waitpid(-1) includes
            // jobs between pipes, not just the one we stored.
            if let Some(i) = job_tracker
                .jobs
                .iter()
                .position(|j| j.pid == wait_status.pid().unwrap())
            {
                // Get correct status message
                match wait_status {
                    WaitStatus::Exited(_pid, ..) => println!(
                        "[{}]+ Done {}",
                        job_tracker.jobs[i].num, job_tracker.jobs[i].cmd
                    ),
                    WaitStatus::Signaled(_pid, signal, ..) => {
                        println!("{} killed by {:?}", _pid, signal)
                    }
                    _ => (),
                };
                job_tracker.jobs.remove(i);
            }
        }

        let (mut cur_command, mut args) = prompt::prompt();
        env_expansion::env_expansion(&mut args);
        tilde_expansion::tilde_expansion(&mut args);

        // Remove ampersand
        if !args.is_empty() && args.last().unwrap() == "&" {
            bg_flag = true;
            args.pop();
            cur_command.pop();
            cur_command.pop();
        }

        match args[0].as_str() {
            "exit" => {
                for job in job_tracker.jobs {
                    waitpid(job.pid, None).unwrap();
                }

                if command_history.is_empty() {
                    println!("No valid commands entered in session. Exiting.");
                } else if command_history.len() < 3 {
                    let last_command = command_history.pop().unwrap();
                    println!("Last valid command: \n{}\nExiting.", last_command);
                } else {
                    println!("Last three valid commands:\n");
                    let mut i = 3;
                    while i > 0 {
                        let command = command_history.pop().unwrap();
                        println!("{}", command);
                        i -= 1;
                    }
                    println!("Exiting.");
                }
                break;
            }
            "cd" => {
                if cd(&mut args) {
                    command_history.push(cur_command);
                }
                continue;
            }
            "jobs" => {
                if args.len() > 1 {
                    println!("Too many arguments: {}", args[1..].join(" "));
                } else if job_tracker.jobs.is_empty() {
                    println!("No active background processes.");
                    command_history.push(cur_command);
                } else {
                    println!("{:<8}| {:<8}|{}", "Job No.", "PID", "Command");
                    for job in job_tracker.jobs.iter() {
                        println!("{:<8}  {:<8} {}", job.num, job.pid, job.cmd);
                    }
                    command_history.push(cur_command);
                }
                continue;
            }
            _ => (),
        }

        let mut cmds: Vec<io_redir::Command> = Vec::new();
        let mut lastpipe = 0;
        let mut last_out: Option<c_int> = None;

        // Piping and Pathsearch
        for (i, arg) in args.iter().enumerate() {
            if arg == "|" {
                let mut command = vec![match path_search::path_search(&args[lastpipe]) {
                    Some(t) => t,
                    _ => {
                        println!("{}: command not found\n", args[lastpipe]);
                        continue;
                    }
                }];
                let mut pipe_args: [c_int; 2] = [0; 2];
                let input: c_int;
                let output: c_int;
                unsafe {
                    nix::libc::pipe(&mut pipe_args[0]);
                }
                (output, input) = (pipe_args[0], pipe_args[1]);

                command.extend(args[lastpipe + 1..i].iter().cloned());
                cmds.push(io_redir::Command {
                    args: command,
                    input: last_out,
                    output: Some(input.into_raw_fd()),
                });
                last_out = Some(output.into_raw_fd());
                lastpipe = i + 1;
            }
        }

        args[lastpipe] = match path_search::path_search(&args[lastpipe]) {
            Some(t) => t,
            _ => {
                println!("{}: command not found\n", args[lastpipe]);
                continue;
            }
        };

        if lastpipe == 0 {
            // no pipe emergency abort to check io redirection and run command
            cmds.push(io_redir::Command {
                args,
                input: None,
                output: None,
            });
        } else {
            cmds.push(io_redir::Command {
                args: args[lastpipe..].to_vec(),
                input: last_out,
                output: None,
            });
        }

        let pid = execute::execute(cmds, bg_flag);
        command_history.push(cur_command.clone());
        if bg_flag {
            job_tracker.push(pid, cur_command);
            println!(
                "{} {}",
                job_tracker.jobs.last().unwrap().num,
                job_tracker.jobs.last().unwrap().pid
            );
            bg_flag = false;
        }
        println!();
    }
}

fn cd(args: &mut [String]) -> bool {
    if args.len() > 2 {
        println!("cd: too many arguments");
        return false;
    }

    let target_path: String = if args.len() == 1 {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        let full = if args[1].starts_with('/') {
            args[1].clone()
        } else {
            let cwd_key = CString::from_str("PWD").unwrap();
            let cwd = unsafe { getenv(cwd_key.as_ptr()) };
            let cwd_str = if cwd.is_null() {
                "/"
            } else {
                unsafe { CStr::from_ptr(cwd) }.to_str().unwrap_or("/")
            };
            format!("{}/{}", cwd_str, args[1])
        };
        let mut stack: Vec<&str> = Vec::new();

        for segment in full.split("/") {
            match segment {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                name => {
                    stack.push(name);
                }
            }
        }
        if stack.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", stack.join("/"))
        }
    };
    let c_target = match CString::new(target_path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let pwd_key = CString::new("PWD").unwrap();

    unsafe {
        let result = chdir(c_target.as_ptr());
        if result == 0 {
            setenv(pwd_key.as_ptr(), c_target.as_ptr(), 1);
            true
        } else {
            let err = Errno::last();
            match err {
                Errno::EACCES => println!("{}: Permission denied.", args[1]),
                Errno::ENOENT => println!("cd: {}: No such file or directory", args[1]),
                Errno::ENOTDIR => println!("cd: {}: Not a directory", args[1]),
                _ => println!("cd: Unspecified error {}", args[1]),
            }
            false
        }
    }
}
