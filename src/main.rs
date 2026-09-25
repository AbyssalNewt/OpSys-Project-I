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

        let mut cmd : io_redir::Command = match io_redir::io_parse(args){
            Ok(com) => {com}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };


        // replace with the built-ins later
        match cmd.args[0].as_str() {
            "exit" => break,
            _ => ()
        }
        cmd.args[0] = match path_search::path_search(&cmd.args[0]) {
            Some(t) => t,
            _ => {println!("{}: command not found\n", cmd.args[0]); continue}
        };

        //println!("Command entered: {}, input file: {}, output file: {}", cmd.args[0], cmd.input.unwrap_or("N/A".to_string()), cmd.output.unwrap_or("N/A".to_string()));

        execute::execute(cmd);



        println!();
    }
}

