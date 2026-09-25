use nix::unistd::{access, AccessFlags};
use std::{env, ffi::{CString}};

pub struct Command {
    pub(crate) args: Vec<String>,
    pub(crate) input: Option<String>,
    pub(crate) output: Option<String>
}
pub fn io_parse(args: Vec<String>) -> Result<Command, String> {
    
    let mut input : Option<String> = None;
    let mut output : Option<String> = None;
    let mut new_args : Option<Vec<String>> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "<" => {

                if !input.is_none() {
                    //Throw error if a second '<' is found.
                    return Err("Error: extra input redirector".to_string())
                }

                if input.is_none() && output.is_none() {
                    //The args of the command are separated from the I/O redirection upon finding I/O redirection
                    new_args = Some(args[..i].to_vec());
                }

                if args[i+1].starts_with("~") {
                    input = Some(env::var("HOME").unwrap_or_default() + "/" + args[i+1].as_str());
                }
                else if !args[i+1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    input = Some(env::var("PWD").unwrap() + "/" + args[i+1].as_str());
                }
                else {
                    //The file path is an absolute path, so we can take it as it is.
                    input = Some(args[i+1].clone());
                }
                let consume_input = input.clone();
                let in_path : CString = CString::new(consume_input.unwrap()).unwrap();
                let inp_cstr = in_path.as_c_str();
                if access(inp_cstr, AccessFlags::R_OK) != Ok(()) {
                    //Checking if the file is readable. The 3 previous lines are shit.
                    return Err("Error: input file is not readable.".to_string());
                }
                i += 1;
            },
            ">" => {
                //Behavior largely the same as for the "<" case.
                if !output.is_none() {
                    return Err("Error: extra output redirector".to_string())
                }

                if input.is_none() && output.is_none()
                {
                    new_args = Some(args[..i].to_vec());
                }

                if args[i+1].starts_with("~") {
                    output = Some(env::var("HOME").unwrap_or_default() + "/" + args[i+1].as_str());
                }
                else if !args[i+1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    output = Some(env::var("PWD").unwrap() + "/" + args[i+1].as_str());
                }
                else {
                    output = Some(args[i+1].clone());
                }
                //I swear to God, there has to be a better way to write this than cloning the Option<String> to consume it, but I do not know Rust yet
                let consume_output = output.clone();
                let out_path : CString = CString::new(consume_output.unwrap()).unwrap();
                let out_cstr = out_path.as_c_str();
                if access(out_cstr, AccessFlags::W_OK) != Ok(())
                {
                    return Err("Error: output file is not writable.".to_string());
                }

                i += 1;
            },
            _  => ()
        }
        i += 1;
    }

    match new_args.is_none()
    {
        true => Ok(Command {args, input, output}),
        false => Ok(Command {args: new_args.unwrap_or_default(), input, output})
    }
}

