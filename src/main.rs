mod env_expansion;
mod path_search;
mod prompt;
mod tilde_expansion;

fn main() {
    loop {
        let mut args = prompt::prompt();
        env_expansion::env_expansion(&mut args);
        tilde_expansion::tilde_expansion(&mut args);

        // replace with the built-ins later
        match args[0].as_str() {
            "exit" => break,
            _ => ()
        }

        let path_to_cmd = match path_search::path_search(&args[0]) {
            Some(t) => t,
            _ => {println!("{}: command not found\n", args[0]); continue}
        };
        println!("{}: command found\n", path_to_cmd)
    }
}
