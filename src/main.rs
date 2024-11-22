use std::io::{stderr, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, io};

fn main() -> Result<(), io::Error> {
    let real_git = env::var("GIT_REAL").expect("Please set GIT_REAL to the original git path.");

    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    let mut old_args = args.into_iter();
    old_args.next(); // pop argv[0]

    let mut cmd = Command::new(&real_git);

    let mut did_modify = false;
    if let Some(argv1) = old_args.next() {
        let varname = format!("GIT_{}_FLAGS", argv1.to_uppercase());
        let var = env::var(&varname);

        cmd.arg(argv1);

        if let Ok(var) = var {
            let new_args = var.split(" ");
            cmd.args(new_args);
            did_modify = true;
        }
    }

    cmd.args(old_args);

    if did_modify {
        eprint!("+ {} ", real_git);
        for arg in cmd.get_args() {
            stderr().write_all(arg.as_bytes()).unwrap();
            eprint!(" ");
        }
        eprintln!();
    }

    return Err(cmd.exec());
}
