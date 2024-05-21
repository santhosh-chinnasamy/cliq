use std::collections::HashMap;
use std::env;
use std::process::{exit, Command};

fn main() {
    let links = HashMap::from([
        ("google", "https://google.com"),
        ("hub", "https://github.com"),
        ("lab", "https://gitlab.com"),
    ]);

    let alias = env::args().nth(1).unwrap_or_else(|| {
        println!("no alias provided");
        exit(1);
    });

    let link = match links.get(alias.as_str()) {
        Some(link) => link,
        None => {
            println!("alias not found");
            exit(1);
        }
    };

    let program = "open";

    Command::new(program).arg(link).spawn().unwrap_or_else(|_| {
        println!("{} command not found", program);
        exit(127);
    });

    println!("Opening {}", link);
}
