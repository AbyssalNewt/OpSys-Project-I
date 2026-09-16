mod env_expansion;
mod prompt;
mod tilde_expansion;

fn main() {
    loop {
        let res = prompt::prompt();
	if res.is_err() && !res.is_ok() {
		println!("{}",res.unwrap());
		std::process::exit(-1);		
	}
    }
}
