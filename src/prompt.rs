use std::env;
use std::io::{self,Write};

pub fn prompt() -> Result<String, io::Error> {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let machine = env::var("MACHINE").unwrap_or_else(|_| "demanitus".to_string());
    let pwd = env::var("MACHINE").unwrap_or_else(|_| "unknown".to_string());
    io::stdout().flush();
    print!("{}@{}:{}> ",user,machine,pwd);
    io::stdout().flush();

    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input)?;

    validate(input.trim().to_string());

    Ok(input)
}

fn validate(cmd: String) {
    let args: Vec<&str> = cmd.split(' ').collect();

    match args[0] {
        "exit"=>std::process::exit(0),
        _=>println!("{}: command not found\n",cmd),
    }
}