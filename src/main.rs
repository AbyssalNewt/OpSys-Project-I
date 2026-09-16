mod env_expansion;
mod prompt;
mod tilde_expansion;

fn main() {
    loop {
        let mut args = prompt::prompt();
        env_expansion::env_expansion(&mut args);
        tilde_expansion::tilde_expansion(&mut args);

        match args[0].as_str() {
            "exit" => break,
            _ => println!("{}: command not found\n", args[0]),
        }
    }
}
