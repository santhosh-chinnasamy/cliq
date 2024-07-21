use std::process::{exit, Command};

const MACOS: &str = "open";
const LINUX: &str = "xdg-open";

fn program() -> String {
    let os: &str = std::env::consts::OS;
    let _program: &str = match os {
        "macos" => MACOS,
        _ => LINUX,
    };

    return _program.to_string();
}

pub fn open(url: String) {
    let binding = program();
    let command = binding.as_str();

    println!("Opening {}", url);

    Command::new(command).arg(url).spawn().unwrap_or_else(|_| {
        eprintln!("{} command not found", command);
        exit(127);
    });
}
