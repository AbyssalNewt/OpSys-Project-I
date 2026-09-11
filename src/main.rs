mod prompt;
fn main() {
    loop {
        let res = prompt::prompt();
	if res.is_err() && !res.is_ok() {
		println!("{}",res.unwrap());
		std::process::exit(-1);		
	}
    }
}
