use std::env;

/// Expands environment variables.
pub fn env_expansion(strings:Vec<&str>) -> Vec<String> {
    strings.iter().map(|s| 
        // Prevent out of bounds slicing
        if s.len() > 0 {
            // Match first char
            match &s[0..1] { 
                "$" => env::var(&s[1..]).unwrap_or_default(),
                _ => s.to_string() }
        } else { 
            "".to_string()
        }).collect()
}
