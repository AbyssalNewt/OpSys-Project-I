use std::{env, ffi::CString};
use nix::libc::{open, O_RDONLY, S_IRUSR, O_WRONLY, O_TRUNC, O_CREAT};

pub struct Command {
    pub(crate) args: Vec<String>,
    pub(crate) input: Option<nix::libc::c_int>,
    pub(crate) output: Option<nix::libc::c_int>,
}
pub fn io_parse(args: Vec<String>) -> Result<Command, String> {
    let mut input: Option<nix::libc::c_int> = None;
    let mut output: Option<nix::libc::c_int> = None;
    let mut new_args: Option<Vec<String>> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "<" => {
                if !input.is_none() {
                    //Throw error if a second '<' is found.
                    return Err("Error: extra input redirector".to_string());
                }

                if input.is_none() && output.is_none() {
                    //The args of the command are separated from the I/O redirection upon finding I/O redirection
                    new_args = Some(args[..i].to_vec());
                }

                if args[i + 1].starts_with("~") {
                    let input_string: CString = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();
                    input =
                        Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    let input_string: CString =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                    input =
                        Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                } else {
                    //The file path is an absolute path, so we can take it as it is.
                    let input_string: CString = CString::new(args[i + 1].clone()).unwrap();
                    input =
                        Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                }
                /* This is actually a race condition! We will check for errors when opening the file.
                let consume_input = input.clone();
                let in_path : CString = CString::new(consume_input.unwrap()).unwrap();
                let inp_cstr = in_path.as_c_str();
                if access(inp_cstr, AccessFlags::R_OK) != Ok(()) {
                    //Checking if the file is readable. The 3 previous lines are shit.
                    return Err("Error: input file is not readable.".to_string());
                }*/
                i += 1;
            }
            ">" => {
                //Behavior largely the same as for the "<" case.
                if !output.is_none() {
                    return Err("Error: extra output redirector".to_string());
                }

                if input.is_none() && output.is_none() {
                    new_args = Some(args[..i].to_vec());
                }

                if args[i + 1].starts_with("~") {
                    let output_string: CString = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();
                    output = Some(unsafe {
                        open(
                            output_string.as_ptr(),
                            O_WRONLY | O_TRUNC | O_CREAT,
                            0o600,
                        )
                    });
                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    let output_string: CString =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                    output = Some(unsafe {
                        open(
                            output_string.as_ptr(),
                            O_WRONLY | O_TRUNC | O_CREAT,
                            0o600,
                        )
                    });
                } else {
                    let output_string: CString = CString::new(args[i + 1].clone()).unwrap();
                    output = Some(unsafe {
                        open(
                            output_string.as_ptr(),
                            O_WRONLY | O_TRUNC | O_CREAT,
                            0o600,
                        )
                    });
                }

                /* While checking for an error would be good, this is actually a race condition
                let consume_output = output.clone();
                let out_path : CString = CString::new(consume_output.unwrap()).unwrap();
                let out_cstr = out_path.as_c_str();
                match access(out_cstr, AccessFlags::F_OK | AccessFlags::W_OK)
                {
                    Err(nix::errno::Errno::EACCES) => return Err("Error: output file is not writable.".to_string()),
                    Ok(()) => (),
                    Err(_) => return Err("Error: Unknown error when checking output file access".to_string())
                }
                */
                i += 1;
            }
            _ => (),
        }
        i += 1;
    }

    match new_args.is_none() {
        true => Ok(Command {
            args,
            input,
            output,
        }),
        false => Ok(Command {
            args: new_args.unwrap_or_default(),
            input,
            output,
        }),
    }
}
