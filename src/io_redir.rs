use nix::libc::{fchmod, fstat, S_IFMT, S_IFREG, S_IXUSR, S_IXGRP, S_IXOTH, close};
use {
    nix::libc::{O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IRUSR, open, stat},
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

                if new_args.is_none() {
                    //The args of the command are separated from the I/O redirection upon finding I/O redirection
                    new_args = Some(args[..i].to_vec())
                }

                let input_string: CString;
                if args[i + 1].starts_with("~") {
                    input_string = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();

                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    input_string =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                } else {
                    //The file path is an absolute path, so we can take it as it is.
                    input_string = CString::new(args[i + 1].clone()).unwrap();
                }
                let fd = unsafe {open(input_string.as_ptr(), O_RDONLY, S_IRUSR)};
                if fd < 0{
                    return Err("Error opening input file".to_string());
                }
                let mut st: stat = unsafe { std::mem::zeroed() };
                let stat_res = unsafe{ fstat(fd, &mut st)};

                if stat_res < 0{
                    return Err("Error stating input file".to_string());
                }

                let is_regular_file = ((st.st_mode & S_IFMT) == S_IFREG) && (st.st_mode & (S_IXUSR | S_IXGRP | S_IXOTH)) == 0;
                if !is_regular_file{
                    unsafe{close(fd);}
                    return Err("Input redirection must be a regular file".to_string());
                }

                input = Some(fd);
                i += 1;
            }
            ">" => {
                //Behavior largely the same as for the "<" case.
                // if !output.is_none() {
                //   return Err("Error: extra output redirector".to_string());
                //}
                let output_string: CString;
                if new_args.is_none() {
                    //The args of the command are separated from the I/O redirection upon finding I/O redirection
                    new_args = Some(args[..i].to_vec())
                }

                if args[i + 1].starts_with("~") {
                    output_string = CString::new(
                        env::var("HOME").unwrap_or_default() + "/" + args[i + 1].as_str(),
                    )
                    .unwrap();
                } else if !args[i + 1].starts_with("/") {
                    //The file path is a relative path, so we are adding the cwd to it.
                    output_string =
                        CString::new(env::var("PWD").unwrap() + "/" + args[i + 1].as_str())
                            .unwrap();
                } else {
                    output_string = CString::new(args[i + 1].clone()).unwrap();

                }
                let fd = unsafe {
                    open(output_string.as_ptr(), O_WRONLY | O_TRUNC | O_CREAT, 0o600)
                };
                unsafe { fchmod(fd, 0o600); }
                output = Some(fd);

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
