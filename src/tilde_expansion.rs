use std::env;

/// Expands tilde to $HOME. Also expands ".." & "." to implement
/// shell-ception via execv.
pub fn tilde_expansion(args: &mut [String]) -> &mut [String] {
    let home = env::var("HOME").unwrap_or_default();
    let pwd = env::var("PWD").unwrap_or_default();

    for arg in args.iter_mut() {
        match arg.as_str() {
            "~" => *arg = home.clone(),
            "." => *arg = pwd.clone(),
            ".." | "../" => *arg = pwd.clone()[..pwd.rfind("/").unwrap()].to_string(),
            // Prevent expansion of relative paths prior to path_search
            // since it might be a path command.
            s if !s.contains("/") => continue,

            s if s.starts_with("~/") => *arg = home.clone() + &arg[1..],
            s if s.starts_with("./") => *arg = pwd.clone() + &arg[1..],
            _ => *arg = pwd.clone() + "/" + arg,
        };
        // Everything at this point is a full-path.
        // Handling ".."
        while let Some(i) = arg.find("/..") {
            *arg = arg[..arg[..i-2].rfind("/").unwrap()].to_string() + &arg[i+3..];
        }
        // Remove extra "/"s
        while let Some(i) = arg.find("//") {
            *arg = arg[..=i].to_string()
        }
        // Remove trailing "/"s
        while arg.len() > 1 && arg.ends_with("/") {
            arg.pop();
        }
    }
    args
}
