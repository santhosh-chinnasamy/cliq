use std::{
    process::{exit, Command, Stdio},
    str,
};

use crate::heimdall;

fn check_git() {
    Command::new("git")
        .arg("--version")
        .spawn()
        .unwrap_or_else(|_| {
            eprintln!("Git not installed");
            exit(127);
        });
}

/* fn check_gitt() {
    match Command::new("git").spawn() {
        Ok(_) => {}
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                println!("`git` command not found");
            } else {
                println!("Some strange error occurred :(");
            }
            exit(127)
        }
    }
} */

fn check_git_init_folder() {
    Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .spawn()
        .unwrap_or_else(|_| {
            eprintln!("git not initialized. run `git init` to initialize git repository");
            exit(127);
        });
}

fn open_remote_url() {
    let result = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let url = str::from_utf8(&result.stdout[..]).unwrap();

    heimdall::open(url.to_string());
}

pub fn main() {
    check_git();
    check_git_init_folder();
    open_remote_url();
}
