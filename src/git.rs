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

    let remote_url = str::from_utf8(&result.stdout[..]).unwrap();
    let is_https = remote_url.starts_with("https");
    let is_ssh = remote_url.starts_with("git@");

    if is_https {
        heimdall::open(remote_url.to_string());
        return;
    }

    if is_ssh {
        heimdall::open(convert_ssh_to_https(remote_url));
        return;
    }

    eprintln!("Invalid git remote");
    exit(127);
}

fn convert_ssh_to_https(url: &str) -> String {
    let mut url = url.replace(":", "/");
    url = url.replace("git@", "https://");
    return url.trim().to_string();
}

pub fn main() {
    check_git();
    check_git_init_folder();
    open_remote_url();
}
