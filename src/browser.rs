use std::process::{exit, Command, Stdio};

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

    let status = Command::new(command)
        .arg(&url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|_| {
            eprintln!("{} command not found", command);
            exit(127);
        });

    if !status.success() {
        eprintln!("unable to open {}", url);
        exit(0);
    }

    println!("Opening {}", url);
}
