use std::os::fd::IntoRawFd;

use nix::libc::{c_int};

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
            "exit" => break,
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

        //println!("Command entered: {}, input file: {}, output file: {}", cmd.args[0], cmd.input.unwrap_or("N/A".to_string()), cmd.output.unwrap_or("N/A".to_string()));
    }
}
