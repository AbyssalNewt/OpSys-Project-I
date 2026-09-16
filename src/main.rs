mod env_expansion;
mod prompt;
mod tilde_expansion;

fn main() {
    loop {
        prompt::prompt().expect("");
    }
}
