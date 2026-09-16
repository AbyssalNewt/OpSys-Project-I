use std::env;

/// Expands environment variables.
pub fn env_expansion(strings:&mut Vec<String>) -> &mut Vec<String>{
    let mut i = 0;
    while i < strings.len() {
        if strings[i].len() > 0 {
            // Match first char
            match &strings[i][0..1] { 
                "$" => strings[i] = env::var(&strings[i][1..]).unwrap_or_default(),
                _ => continue}
        }
        i += 1;
    }
    strings
}
