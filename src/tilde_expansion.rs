use std::{cmp, env};

/// Expands tilde to $HOME.
pub fn tilde_expansion(args: &mut Vec<String>) -> &mut Vec<String> {
    let home = env::var("HOME").unwrap_or_default();

    for arg in args.iter_mut() {
        if arg.starts_with("~/") {
            *arg = home.to_string() + &arg[1..];
        } else if arg.starts_with("~") {
            *arg = home.to_string();
        }
    }
    args
}
