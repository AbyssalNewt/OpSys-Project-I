use std::env;
use std::ffi::{CString, CStr};
use std::os::fd::IntoRawFd;
use std::str::FromStr;
use nix::errno::Errno;
use nix::libc::{c_int, chdir, getenv, setenv};

mod env_expansion;
mod path_search;
mod prompt;
mod tilde_expansion;

mod execute;
mod io_redir;

fn main() {
    loop {
        let mut args = prompt::prompt();
        env_expansion::env_expansion(&mut args);
        tilde_expansion::tilde_expansion(&mut args);

        /*let mut cmd : io_redir::Command = match io_redir::io_parse(args){
            Ok(com) => {com}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };*/

        // replace with the built-ins later
        match args[0].as_str() {
            "exit" => {break
            },
            "cd" => {
                cd(&mut args);
                continue;

            },
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

        execute::execute(cmds);
        println!();

        //println!("Command entered: {}, input file: {}, output file: {}", cmd.args[0], cmd.input.unwrap_or("N/A".to_string()), cmd.output.unwrap_or("N/A".to_string()));
    }
}

fn cd(args: &mut Vec<String>) {


    if args.len() > 2 {
        println!("cd: too many arguments");
        return;
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
        Err(_) => return
    };
    let pwd_key = CString::new("PWD").unwrap();

    unsafe {


        let result = chdir(c_target.as_ptr());
        if result == 0 {

            setenv(pwd_key.as_ptr(), c_target.as_ptr(), 1);
        } else {
            let err = Errno::last();
            match err
            {
                Errno::EACCES => println!("{}: Permission denied.", args[1]),
                Errno::ENOENT => println!("cd: {}: No such file or directory", args[1]),
                Errno::ENOTDIR => println!("cd: {}: Not a directory", args[1]),
                _ => println!("cd: Unspecified error {}", args[1]),
            }
        }
    };
}
