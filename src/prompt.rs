use crate::env_expansion::*;
use crate::tilde_expansion::*;
use std::env;
use std::io::{self,Write};

pub fn prompt() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let machine = env::var("MACHINE").unwrap_or_else(|_| "demanitus".to_string());
    let pwd = env::var("PWD").unwrap_or_else(|_| "unknown".to_string());

    io::stdout().flush().unwrap();
    print!("{}@{}:{}> ",user,machine,pwd);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input).unwrap();

    validate(input.trim().to_string());

    input
}

fn validate(cmd: String) {
    let mut args: Vec<String> = cmd.split(' ').map(|s| s.to_string()).collect();
    args = env_expansion(args);
    args = tilde_expansion(args);

    match args[0].as_str() {
        "exit"=>std::process::exit(0),
        _=>println!("{}: command not found\n", args.join(" ")),
    }
}
