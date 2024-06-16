use clap::{Parser, Subcommand};
mod alias;

#[derive(Parser)]
#[command(name = "cliq", version, about , author, long_about = None)]
struct Cliq {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // to handle aliases in toml file
    #[command(external_subcommand)]
    #[allow(dead_code)]
    Options(Vec<String>),
}

fn main() {
    let cliq = Cliq::parse();

    match &cliq.command {
        Commands::Options(args) => {
            let input = args.join(" ");
            let url = alias::link(input);
            alias::open(url);
        }
    }
}
