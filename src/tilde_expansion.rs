use std::{cmp,env};

/// Expands tilde to $HOME.
pub fn tilde_expansion(strings:&mut Vec<String>) -> &mut Vec<String> {
    let home = env::var("HOME").unwrap_or_default();
    let mut i = 0;

    while i < strings.len() {
        // calc min to prevent out of bounds slicing
        let min = cmp::min(2, strings[i].len());
        if &strings[i][..min] == &"~/"[..min] {
            strings[i] = home.clone() + &strings[i][min..];
        }
        i += 1;
    }
    strings
}
