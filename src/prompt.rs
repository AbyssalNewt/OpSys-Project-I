use std::{
    env,
    io::{self, Write},
};

/// Prints greeting prompt & turns input into arguments for further parsing.
pub fn prompt() -> (String, Vec<String>) {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let machine = env::var("MACHINE").unwrap_or_else(|_| "demanitus".to_string());
    let pwd = env::var("PWD").unwrap_or_else(|_| "unknown".to_string());

    // Flush buffers to force output
    io::stdout().flush().unwrap();
    print!("{}@{}:{}> ", user, machine, pwd);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input).unwrap();

    (
        input.trim().to_string(),
        input.trim().split(' ').map(|s| s.to_string()).collect(),
    )
}
