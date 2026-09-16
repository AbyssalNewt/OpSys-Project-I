use std::env;

/// Expands tilde to $HOME.
pub fn tilde_expansion(strings:Vec<String>) -> Vec<String> {
    let home = env::var("HOME").unwrap_or_default();
    strings.iter().map(|s| 
        match s.len() {
            0 => s.to_string(),
            1 => match s.as_str() { "~" => home.to_string(), _ => s.to_string() },
            _ => match &s[0..2] { "~/" => home.to_string() + &s[1..], _ => s.to_string() },
        }).collect()
}
