use std::{fs, process::exit};

use toml::Table;

const CONFIG_FOLDER: &str = ".config/cliq";
const CONFIG_FILE: &str = "cliq.toml";
const DEFAULT_CONFIG: &str = "
[links]
google = \"https://google.com\"
hub = \"https://github.com\"
lab = \"https://gitlab.com\"
";

fn config_file() -> String {
    let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let cliq_config = format!("{}/{}/{}", home_dir, CONFIG_FOLDER, CONFIG_FILE);
    cliq_config.to_string()
}

fn create_config() {
    let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let cliq_folder = format!("{}/{}", home_dir, CONFIG_FOLDER);
    std::fs::create_dir_all(&cliq_folder).unwrap();

    let cliq_config = format!("{}/{}", cliq_folder, CONFIG_FILE);
    std::fs::File::create(&cliq_config).unwrap();

    // add content to file
    std::fs::write(cliq_config, DEFAULT_CONFIG).unwrap();
}

fn read_config() -> Table {
    let is_config_exist = std::path::Path::new(&config_file()).exists();
    if !is_config_exist {
        create_config();
    }

    let cliq_config = match fs::read_to_string(config_file()) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading cliq.toml. \nCreate cliq.toml file under $HOME/.config/cliq folder. {}", e);
            exit(1);
        }
    };

    let cliq_data: Table = cliq_config.parse().unwrap();
    cliq_data
}

pub fn links() -> Table {
    let cliq_data = read_config();
    let links = cliq_data["links"].as_table().unwrap().clone();
    links
}

pub fn link(alias: String) -> String {
    let links = links();
    let link = match links.get(alias.as_str()) {
        Some(link) => link.as_str().unwrap(),
        None => {
            eprintln!("alias not found in cliq config");
            exit(1);
        }
    };

    link.to_string()
}
