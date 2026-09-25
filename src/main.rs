use nix::libc::{STDIN_FILENO, STDOUT_FILENO};

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

        for (i, arg) in args.iter_mut().enumerate() {
            if arg == "|" {
                cmds.push(io_redir::Command {
                    args: {
                        vec![match path_search::path_search(&args[lastpipe]) {
                            Some(t) => t,
                            _ => {
                                println!("{}: command not found\n", args[lastpipe]);
                                continue;
                            }
                        }]
                        .append(args[lastpipe..i - 1].to_vec())
                    },
                    input: None,
                    output: None,
                });

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

        if (lastpipe == 0) {
            // no pipe emergency abort to check io redirection and run command
            let cmd: io_redir::Command = match io_redir::io_parse(args) {
                Ok(com) => com,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
            cmds.push(cmd);
        } else {
            cmds.push(io_redir::Command {
                args: args[lastpipe..].to_vec(),
                input: None,
                output: None,
            });
        }

        execute::execute(cmds);

        //println!("Command entered: {}, input file: {}, output file: {}", cmd.args[0], cmd.input.unwrap_or("N/A".to_string()), cmd.output.unwrap_or("N/A".to_string()));
    }
}
