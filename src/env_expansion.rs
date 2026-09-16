use std::env;

/// Expands environment variables.
pub fn env_expansion(args:&mut Vec<String>) -> &mut Vec<String>{
    let mut i = 0;
    while i < args.len() {
        if args[i].len() > 0 {
            // Match first char
            match &args[i][0..1] { 
                "$" => args[i] = env::var(&args[i][1..]).unwrap_or_default(),
                _ => continue}
        }
        i += 1;
    }
    args
}
