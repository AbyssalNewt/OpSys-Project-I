# COP4610 Project 1: Build a shell with fancy features

## Contributors:

- Austin Works
- Mason Simmons
- Niko Krinos

## Division of Labor

- Prompt
    - Niko Krinos

- Environment Variables
    - Austin Works

- Tilde Expansion
    - Austin Works
    
- $PATH Search
    - Austin Works

- External Command Execution
    - Niko Krinos

- I/O Redirection
    - Mason Simmons

- Piping
    - Niko Krinos
    - Austin Works

- Background Processing
    - Mason Simmons

- Internal Command Execution
    - Mason Simmons

- Extra Credit
    - Unlimited Pipes - Niko Krinos
    - Piping & I/O - Austin Works
    - Shell-ception - Mason Simmons

## File Listing

- src
    - env_expansion.rs
    - execute.rs
    - io_redir.rs
    - main.rs
    - path_search.rs
    - prompt.ps
    - tilde_expansion.rs
- .envrc
- .gitignore
- Cargo.lock
- Cargo.toml
- shell.nix
- README.md

## How to Compile and Run

1. Download all of the source code
2. Open a shell with rust and cargo installed in the root of the source code
3. Run `cargo run` in your shell.

## Development Logs

### Austin Works
- Wed Sep 9 13:09:11 2026 -0400
    - feat(nix-shell): adds rustc, cargo
- Wed Sep 9 13:20:08 2026 -0400
    - chore: adds direnv to .gitignore
- Wed Sep 9 13:21:05 2026 -0400
    - build: adds .envrc to run shell.nix
- Wed Sep 9 13:23:26 2026 -0400
    - fix: fixes typo in .gitignore
- Thu Sep 10 16:02:05 2026 -0400
    - chore: removes .idea and ignores idea/*
- Thu Sep 10 16:42:55 2026 -0400
    - chore: adds test.sh to .gitignore
- Thu Sep 10 16:48:12 2026 -0400
    - fix: typo MACHINE -> PWD
- Wed Sep 16 00:52:00 2026 -0400
    - feat: adds environment variable and tilde expansions
- Wed Sep 16 01:04:01 2026 -0400
    - Merge branch 'main' into expansions
- Wed Sep 16 01:06:57 2026 -0400
    - fix(tilde_expansion): fixes ~/* expansions
- Wed Sep 16 09:59:17 2026 -0400
    - feat(env_expansion): parameter &str -> String
- Wed Sep 16 10:00:18 2026 -0400
    - fix: changes initial args type &str -> String
- Wed Sep 16 10:20:25 2026 -0400
    - style: fixes indentation
- Wed Sep 16 10:23:12 2026 -0400
    - feat: imports path_search and execute
- Wed Sep 16 12:17:21 2026 -0400
    - fix: removes execv module
- Wed Sep 16 13:22:40 2026 -0400
    - feat(prompt): handles unrecoverable errors by panicing via unwrap
- Wed Sep 16 14:02:44 2026 -0400
    - feat(env_expansion): now mutates and returns
- Wed Sep 16 14:15:27 2026 -0400
    - feat(tilde_expansion): now mutates and returns
- Wed Sep 16 14:31:11 2026 -0400
    - feat: renames parameter strings -> args
- Wed Sep 16 14:39:54 2026 -0400
    - refactor(prompt): moves control loop to main
- Wed Sep 16 14:47:33 2026 -0400
    - fix: fixed continue skipping loop increment
- Wed Sep 16 14:56:27 2026 -0400
    - feat: mirrors bash command not found message
- Wed Sep 16 15:02:56 2026 -0400
    - fix: prevents empty string expansion
- Wed Sep 16 15:06:30 2026 -0400
    - build: imports nix with features fs and process
- Wed Sep 16 15:18:43 2026 -0400
    - fix(tilde_expansion): keeps trailing '/'
- Wed Sep 16 16:44:53 2026 -0400
    - feat: adds path search
- Tue Sep 22 21:36:56 2026 -0400
    - golf (making code shorter)
- Tue Sep 22 22:36:43 2026 -0400
    - feat(main): adds newline after executing command
- Wed Sep 23 13:59:58 2026 -0400
    - build(shell.nix): added bacon and clippy
- Wed Sep 23 14:01:07 2026 -0400
    - refactor(execute): removed unnecessary return
- Thu Sep 24 19:43:54 2026 -0400
    - refactor(env_expansion): golf
- Thu Sep 24 20:40:40 2026 -0400
    - refactor(tilde_expansion): improved readability
- Sat Sep 26 15:14:40 2026 -0400
    - fix(main): fixes overabundance of pipes
- Sat Sep 26 17:16:10 2026 -0400
    - fix(pipes): pipes now work
- Sat Sep 26 18:33:54 2026 -0400
    - fix: fixes hanging by closing write fds
- Sat Sep 26 18:34:29 2026 -0400
    - fix: corrects off by one error causing incomplete commands
- Sat Sep 26 18:45:12 2026 -0400
    - fix: removed test messages
- Sat Sep 26 18:46:33 2026 -0400
	- feat: adds newline at end for legibility

### Mason Simmons
- Date:   Fri Sep 25 04:06:38 2026 -0400
    - added io_redir.rs, first draft io_parse() & Command struct
- Date:   Fri Sep 25 07:24:18 2026 -0400
    - big changes made to execute.rs, fixes to io_redir
- Date:   Sat Sep 26 18:16:27 2026 -0400
    - removed file not found error in execute and dead commented code
- Date:   Sat Sep 26 18:17:36 2026 -0400
    - Merge remote-tracking branch 'origin/main'
- Date:   Sun Sep 27 00:06:33 2026 -0400
    - cd partially works, needs to properly set pwd and move logically
- Date:   Sun Sep 27 00:06:45 2026 -0400
    - Merge remote-tracking branch 'origin/main'

### Niko Krinos
- Date:   Wed Sep 9 11:50:16 2026 -0400
    - init comit
- Date:   Thu Sep 10 09:44:50 2026 -0400
    - fix misspelled shell.nix
- Date:   Thu Sep 10 10:27:34 2026 -0400
    - added prompt
- Date:   Thu Sep 10 11:37:11 2026 -0400
    - error handing is hard?
- Date:   Fri Sep 11 14:01:42 2026 -0400
    - actually printing out error messages now and started thinking about External program Execution
- Date:   Fri Sep 18 14:08:32 2026 -0400
    - updated execute.rs to use nix module.
- Date:   Mon Sep 21 17:58:42 2026 -0400
    - broken code will fix soon, learning which types are correct
- Date:   Fri Sep 25 15:10:19 2026 -0400
    - change io redir to pass FDs instead of CStrings
- Date:   Fri Sep 25 16:53:23 2026 -0400
    - Piping is mostly done just need to fix execute.rs to pass inputs if array length is > 1
- Date:   Fri Sep 25 20:43:43 2026 -0400
    - fixed one error but still need finish piping
- Date:   Sat Sep 26 00:41:08 2026 -0400
    - mostly working pipe

## Group Meetings
- September 18th, 2026
    - Pair programming
- September 25th, 2026
    - Pair programming

## Extra Credit
- Implemented:
    - Support unlimited number of pipes
    - Support piping and I/O redirection in a single command
    - Shell-ception
