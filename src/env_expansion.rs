use std::env;

/// Expands environment variables.
pub fn env_expansion(args:&mut Vec<String>) -> &mut Vec<String>{
    for arg in args.iter_mut() {
        if arg.starts_with("$") {
            *arg = env::var(&arg[1..]).unwrap_or_default()
        }
    }
    args
}
