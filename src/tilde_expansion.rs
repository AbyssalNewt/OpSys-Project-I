use std::{env};

/// Expands tilde to $HOME.
pub fn tilde_expansion(args: &mut Vec<String>) -> &mut Vec<String> {
    let home = env::var("HOME").unwrap_or_default();
    let pwd = env::var("PWD").unwrap_or_default();

    for arg in args.iter_mut() {
        if arg.starts_with("~/") {
            *arg = home.to_string() + &arg[1..];
        } else if arg.starts_with("~") {
            *arg = home.to_string();
        } else if arg.starts_with("../") {
            *arg = pwd.split_at(pwd.rfind("/").unwrap()).0.to_string() + &arg[1..];
        } else if arg.starts_with("./") {
            *arg = pwd.to_string() + &arg[1..];
        } else if arg.starts_with("..") {
            *arg = pwd.split_at(pwd.rfind("/").unwrap()).0.to_string();
        } else if arg.starts_with(".") {
            *arg = pwd.to_string();
        }
    }
    args
}
