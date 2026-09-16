use std::{cmp,env};

/// Expands tilde to $HOME.
pub fn tilde_expansion(args:&mut Vec<String>) -> &mut Vec<String> {
    let home = env::var("HOME").unwrap_or_default();
    let mut i = 0;

    while i < args.len() {
        // calc min to prevent out of bounds slicing
        let min = cmp::min(2, args[i].len());
        if min > 0 && &args[i][..min] == &"~/"[..min] {
            args[i] = home.clone() + &args[i][1..];
        }
        i += 1;
    }
    args
}
