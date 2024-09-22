use clap::{Parser, Subcommand};
mod alias;
mod browser;
mod commands;

#[derive(Parser)]
#[command(name = "cliq", version, about, author, long_about = None)]
struct CliqArgs {
    #[command(subcommand)]
    command: Cliq,
}

#[derive(Subcommand, Debug)]
enum Cliq {
    #[command(about = "get all aliases from cliq.toml", aliases = ["ls"])]
    List,
    #[command(about = "open remote git repository", aliases = ["g"])]
    Git(commands::git::GitArgs),
    /// to handle aliases in toml file
    #[command(external_subcommand)]
    #[allow(dead_code)]
    Options(Vec<String>),
}

fn main() {
    let cliq = CliqArgs::parse();

    match &cliq.command {
        Cliq::List => {
            let parsed_toml = alias::links();
            print!("{}", parsed_toml);
        }
        Cliq::Git(args) => commands::git::main(args),
        Cliq::Options(args) => {
            let input = args.join(" ");
            let url = alias::link(input);
            browser::open(url);
        }
    }
}
