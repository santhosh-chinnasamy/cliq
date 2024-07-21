use std::{
    process::{exit, Command, Stdio},
    str,
};

use crate::browser;

fn check_git() {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Git not installed");
            exit(127);
        });
}

fn check_git_init_folder() {
    let status = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("git not initialized.\nrun `git init` to initialize git");
        exit(0);
    }
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
    let is_http = remote_url.starts_with("http");
    let is_ssh = remote_url.starts_with("git@");

    if is_http {
        browser::open(remote_url.to_string());
        return;
    }

    if is_ssh {
        browser::open(convert_ssh_to_https(remote_url));
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
