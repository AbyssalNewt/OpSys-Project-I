use std::env;
use std::ffi::{CString, CStr};
use std::os::fd::IntoRawFd;
use std::str::FromStr;
use nix::errno::Errno;
use nix::libc::{c_int, chdir, getenv, setenv};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;

mod env_expansion;
mod path_search;
mod prompt;
mod tilde_expansion;
mod execute;
mod io_redir;

#[derive(Clone)]
pub struct JobTracker {
    last_job:i32, //or c_int or whatever
    cur_command:String,
    jobs : Vec<(i32, nix::unistd::Pid, String)> //job number, pid, and command
}
impl JobTracker {
    fn push(&mut self, pid :nix::unistd::Pid) {
        self.last_job += 1;
        self.jobs.push((self.last_job,pid,self.cur_command.clone()));
    }
}

fn main() {

    let mut job_tracker = JobTracker{
        last_job : 0,
        cur_command : String::new(),
        jobs : Vec::new()
    };

    let mut command_history: Vec<Vec<String>> = Vec::new();

    let mut bg_flag : bool = false;

    loop {
        loop {
            match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::Exited(_pid, status)) => {
                    if let Some(job_index) = job_tracker.jobs.iter().position(|(_, pid, _)| *pid == _pid) {
                        println!("{} done: {}", job_tracker.jobs[job_index].0, job_tracker.jobs[job_index].2);
                        job_tracker.jobs.remove(job_index);
                    }
                },
                Ok(WaitStatus::Signaled(_pid, signal, core_dumped)) => {
                    if let Some(job_index) = job_tracker.jobs.iter().position(|(_, pid, _)| *pid == _pid) {
                        println!("{} killed by {:?}", _pid, signal);
                        job_tracker.jobs.remove(job_index);
                    }
                },
                Ok(WaitStatus::StillAlive) => {
                    //All background processes are active.
                    break;
                },
                Err(nix::errno::Errno::ECHILD) => {
                    // No child processes to check.
                    break;
                },
                _ => {}
            }
        }

        let mut args = prompt::prompt();
        job_tracker.cur_command = args.join(" ");
        env_expansion::env_expansion(&mut args);
        tilde_expansion::tilde_expansion(&mut args);

        /*let mut cmd : io_redir::Command = match io_redir::io_parse(args){
            Ok(com) => {com}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };*/

        if !args.is_empty() && args.last().unwrap() == "&"{
            bg_flag = true;
            args.pop();
            job_tracker.cur_command.pop();
        }

        // replace with the built-ins later
        match args[0].as_str() {
            "exit" => {

                //TODO: waitpid on the jobs list until they all finish

                if command_history.is_empty()
                {
                    println!("No valid commands entered in session. Exiting.");
                }
                else if command_history.len() < 3
                {
                    let last_command = command_history.pop().unwrap();
                    println!("Last valid command: \n{}\nExiting.", last_command.join(" "));
                }
                else{
                    println!("Last three valid commands:\n");
                    let mut i = 3;
                    while i > 0{
                        let command = command_history.pop().unwrap().join(" ");
                        println!("{}", command);
                        i = i - 1;
                    }
                    println!("Exiting.");
                }
                break
            },
            "cd" => {
                if cd(&mut args){
                    command_history.push(args);
                }
                continue;

            },
            "jobs" =>
                {
                    if args.len() > 1{
                        println!("Too many arguments: {}", args[1..].join(" "));
                    }
                    else if job_tracker.jobs.is_empty()
                    {
                        println!("No active background processes.");
                        command_history.push(args);
                    }
                    else {
                        println!("{:<8}| {:<8}|{}", "Job No.", "PID", "Command");
                        for job in job_tracker.jobs.iter()
                        {
                            println!("{:<8}  {:<8} {}", job.0, job.1, job.2);
                        }
                        command_history.push(args);
                    }
                    continue;
                }
            _ => (),
        }

        let mut cmds: Vec<io_redir::Command> = Vec::new();
        let mut lastpipe = 0;
        let mut last_out: Option<c_int> = None;

        for (i, arg) in args.iter().enumerate() {
            let command = match path_search::path_search(&args[lastpipe]) {
                Some(t) => t,
                _ => {
                    println!("{}: command not found\n", args[lastpipe]);
                    continue;
                }
            };
            let mut stupid = vec![command];
            if arg == "|" {
                let mut pipe_args : [c_int;2] = [0;2];
                let input : c_int;
                let output: c_int;
                unsafe {nix::libc::pipe(&mut pipe_args[0]);}
                (output, input) = (pipe_args[0], pipe_args[1]);

                stupid.extend(args[lastpipe+1..i].iter().cloned());
                cmds.push(io_redir::Command {
                    args: stupid,
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

        //TODO: determine whether to add command to command history from execute::execute
        execute::execute(cmds, bg_flag, &mut job_tracker);
        bg_flag = false;
        println!();

        //println!("Command entered: {}, input file: {}, output file: {}", cmd.args[0], cmd.input.unwrap_or("N/A".to_string()), cmd.output.unwrap_or("N/A".to_string()));
    }
}

fn cd(args: &mut Vec<String>) -> bool {


    if args.len() > 2 {
        println!("cd: too many arguments");
        return false;
    }

    let target_path: String = if args.len() == 1 {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        let full = if args[1].starts_with('/'){
            args[1].clone()
        }
        else{
            let cwd_key = CString::from_str("PWD").unwrap();
            let cwd = unsafe { getenv(cwd_key.as_ptr()) };
            let cwd_str = if cwd.is_null() {
                "/"
            }
                else{
                    unsafe { CStr::from_ptr(cwd) }.to_str().unwrap_or("/")
                };
            format!("{}/{}", cwd_str, args[1])
        };
        let mut stack: Vec<&str> = Vec::new();

        for segment in full.split("/") {
            match segment{
                "" | "." => {},
                ".." => {stack.pop();},
                name => {stack.push(name);}
            }
        }
        if stack.is_empty(){
            "/".to_string()
        }
        else {
            format!("/{}", stack.join("/"))
        }
    };
    let c_target = match CString::new(target_path)
    {
        Ok(c) => c,
        Err(_) => return false
    };
    let pwd_key = CString::new("PWD").unwrap();

    unsafe {


        let result = chdir(c_target.as_ptr());
        if result == 0 {
            setenv(pwd_key.as_ptr(), c_target.as_ptr(), 1);
            true
        } else {
            let err = Errno::last();
            match err
            {
                Errno::EACCES => println!("{}: Permission denied.", args[1]),
                Errno::ENOENT => println!("cd: {}: No such file or directory", args[1]),
                Errno::ENOTDIR => println!("cd: {}: Not a directory", args[1]),
                _ => println!("cd: Unspecified error {}", args[1]),
            }
            false
        }
    }
}

