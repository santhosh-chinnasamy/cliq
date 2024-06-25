use clap::{Parser, Subcommand};
mod alias;
mod git;
mod heimdall;

#[derive(Parser)]
#[command(name = "cliq", version, about, author, long_about = None)]
struct Cliq {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "get all aliases from cliq.toml", aliases = ["ls"])]
    List,
    #[command(about = "open remote git repository", aliases = ["g"])]
    Git,
    /// to handle aliases in toml file
    #[command(external_subcommand)]
    #[allow(dead_code)]
    Options(Vec<String>),
}

fn main() {
    let cliq = Cliq::parse();

    match &cliq.command {
        Commands::List => {
            let parsed_toml = alias::links();
            print!("{}", parsed_toml);
        }
        Commands::Git => git::main(),
        Commands::Options(args) => {
            let input = args.join(" ");
            let url = alias::link(input);
            heimdall::open(url);
        }
    }
}
