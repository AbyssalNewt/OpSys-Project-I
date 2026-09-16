mod env_expansion;
mod path_search;
mod prompt;
mod tilde_expansion;

fn main() {
    loop {
        let res = prompt::prompt();
    }
}
