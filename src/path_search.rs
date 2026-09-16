use std::env::var;
use nix::unistd::{access, AccessFlags};

/// Searches the path for the 
pub fn path_search(cmd: &String) -> Option<String>{
    let path = var("PATH").unwrap_or_default();

    for p in path.split(":") {
        let full_path = p.to_string() + "/" + cmd;
        match access( full_path.as_str(), AccessFlags::F_OK ) {
            Ok(()) => return Some(full_path),
            _ => ()
        }
    } 
    None
}
