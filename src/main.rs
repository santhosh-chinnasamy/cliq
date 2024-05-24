use std::process::{exit, Command};
use std::{env, fs, path};
use toml::Table;

fn main() {
    let home_dir = match env::var_os("HOME") {
        Some(val) => val,
        None => {
            eprintln!("$HOME dir not defined in the environment.");
            exit(1);
        }
    };

    let file_path = path::Path::new(&home_dir).join(".config/cliq/cliq.toml");

    let cliq_config = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading cliq.toml. \nCreate cliq.toml file under $HOME/.config/cliq folder. {}", e);
            exit(1);
        }
    };

    let cliq_data: Table = cliq_config.parse().unwrap();
    let links = &cliq_data["links"].as_table().unwrap().clone();

    let alias = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("no alias provided");
        exit(1);
    });

    let link = match links.get(alias.as_str()) {
        Some(link) => link.as_str().unwrap(),
        None => {
            eprintln!("alias not found");
            exit(1);
        }
    };

    let program = "open";

    Command::new(program).arg(link).spawn().unwrap_or_else(|_| {
        eprintln!("{} command not found", program);
        exit(127);
    });
    println!("Opening {}", link);
}
