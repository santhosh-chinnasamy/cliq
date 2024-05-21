use std::collections::HashMap;
use std::env;
use std::process::Command;

fn main() {
    let links = HashMap::from([
        ("google", "https://google.com"),
        ("hub", "https://github.com"),
        ("lab", "https://gitlab.com"),
    ]);

    let alias = env::args().nth(1).expect("no alias provided");

    let link = match links.get(alias.as_str()) {
        Some(link) => link,
        None => panic!("alias not found"),
    };

    println!("Opening {}", &link);

    Command::new("open")
        .arg(link)
        .spawn()
        .expect("open command failed to execute");
}
