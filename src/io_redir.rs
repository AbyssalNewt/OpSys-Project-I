use {
    nix::libc::{O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IRUSR, open},
    std::{env, ffi::CString},
};

// A struct representing a command which stores the arguments
// and the file descriptors for which should replace stdin & stdout.
pub struct Command {
    pub(crate) args: Vec<String>,
    pub(crate) input: Option<nix::libc::c_int>,
    pub(crate) output: Option<nix::libc::c_int>,
}

// Parse "<" ">" symbols into a Commands input and/or output
pub fn io_parse(
    args: Vec<String>,
    input: Option<nix::libc::c_int>,
    output: Option<nix::libc::c_int>,
) -> Result<Command, String> {
    let mut input: Option<nix::libc::c_int> = input;
    let mut output: Option<nix::libc::c_int> = output;
    let mut new_args: Option<Vec<String>> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "<" => {
                //if !input.is_none() {
                //Throw error if a second '<' is found.
                //  return Err("Error: extra input redirector".to_string());
                //}

                if input.is_none() && output.is_none() {
                    //The args of the command are separated from the I/O redirection upon finding I/O redirection
                    new_args = Some(args[..i].to_vec());
                }

                if args[i + 1].starts_with("~") {
                    let input_string: CString = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();
                    input = Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    let input_string: CString =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                    input = Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                } else {
                    //The file path is an absolute path, so we can take it as it is.
                    let input_string: CString = CString::new(args[i + 1].clone()).unwrap();
                    input = Some(unsafe { open(input_string.as_ptr(), O_RDONLY, S_IRUSR) });
                }

                i += 1;
            }
            ">" => {
                //Behavior largely the same as for the "<" case.
                // if !output.is_none() {
                //   return Err("Error: extra output redirector".to_string());
                //}

                if input.is_none() && output.is_none() {
                    new_args = Some(args[..i].to_vec());
                }

                if args[i + 1].starts_with("~") {
                    let output_string: CString = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();
                    output = Some(unsafe {
                        open(output_string.as_ptr(), O_WRONLY | O_TRUNC | O_CREAT, 0o600)
                    });
                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    let output_string: CString =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                    output = Some(unsafe {
                        open(output_string.as_ptr(), O_WRONLY | O_TRUNC | O_CREAT, 0o600)
                    });
                } else {
                    let output_string: CString = CString::new(args[i + 1].clone()).unwrap();
                    output = Some(unsafe {
                        open(output_string.as_ptr(), O_WRONLY | O_TRUNC | O_CREAT, 0o600)
                    });
                }

                i += 1;
            }
            _ => (),
        }
        i += 1;
    }

    match new_args {
        None => Ok(Command {
            args,
            input,
            output,
        }),
        Some(a) => Ok(Command {
            args: a,
            input,
            output,
        }),
    }
}
