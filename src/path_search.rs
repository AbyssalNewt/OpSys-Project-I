use {
    nix::unistd::{AccessFlags, access},
    std::env::var,
};

/// Searches the path for the given command and returns its full path
/// if it is found.
pub fn path_search(cmd: &String) -> Option<String> {
    if cmd.is_empty() {
        return None;
    }

    // Prevent searches on absolute paths.
    if &cmd[0..1] == "/" {
        return Some(cmd.to_string());
    };

    let path = var("PATH").unwrap_or_default();

    for p in path.split(":") {
        let full_path = p.to_string() + "/" + cmd;
        if access(full_path.as_str(), AccessFlags::F_OK).is_ok() {
            return Some(full_path);
        }
    }
    None
}
