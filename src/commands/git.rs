use std::{
    process::{exit, Command, Stdio},
    str,
};

use clap::Args;

use crate::browser;

#[derive(Args, Debug)]
pub struct GitArgs {
    /// opens given remote branch
    #[arg(short, long)]
    branch: Option<String>,

    /// opens given remote commit
    #[arg(short, long)]
    commit: Option<String>,

    /// opens given remote tag
    #[arg(short, long)]
    tag: Option<String>,
}

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

fn open_remote_url(branch: &str) {
    let result = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let repo_url = str::from_utf8(&result.stdout[..]).unwrap();
    let is_http = repo_url.starts_with("http");
    let is_ssh = repo_url.starts_with("git@");
    // add branch to remote url
    let mut remote_url = format!("{}/tree/{}", repo_url.trim(), branch);
    remote_url = remote_url.replace(".git", "");

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

fn convert_ssh_to_https(url: String) -> String {
    let mut url = url.replace(":", "/");
    url = url.replace("git@", "https://");
    return url.trim().to_string();
}

fn get_current_branch() -> String {
    let result = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let branch = str::from_utf8(&result.stdout[..]).unwrap().to_string();
    branch
}

fn get_current_tag_name() -> String {
    let result = Command::new("git")
        .arg("describe")
        .arg("--tags")
        .arg("exact-match")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let tag = str::from_utf8(&result.stdout[..]).unwrap().to_string();
    tag
}

fn get_current_commit_sha() -> String {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let commit_sha = str::from_utf8(&result.stdout[..]).unwrap().to_string();
    commit_sha
}

fn get_branch_or_tag_or_commit() -> String {
    let branch = get_current_branch();
    let tag = get_current_tag_name();
    let commit = get_current_commit_sha();

    if !branch.is_empty() {
        branch
    } else if !tag.is_empty() {
        return tag;
    } else if !commit.is_empty() {
        return commit;
    } else {
        eprintln!("No branch, tag or commit found");
        exit(127);
    }
}

pub fn main(args: &GitArgs) {
    check_git();
    check_git_init_folder();

    if args.branch.is_some() {
        let branch = args.branch.as_ref().unwrap();
        open_remote_url(branch);
        return;
    }

    if args.tag.is_some() {
        let tag = args.tag.as_ref().unwrap();
        open_remote_url(tag);
        return;
    }

    if args.commit.is_some() {
        let commit = args.commit.as_ref().unwrap();
        open_remote_url(commit);
        return;
    }

    let fallback: String = get_branch_or_tag_or_commit();
    open_remote_url(fallback.trim());
}
