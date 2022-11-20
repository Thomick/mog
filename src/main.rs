use clap::Parser;
use clap::Subcommand;

mod object;
mod repository;
mod utils;
use repository::Repository;

#[derive(Parser)]
#[command(name = "mog")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<String> },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init { path } => match path {
            Some(path) => Repository::create_repo(&path).unwrap(),
            None => Repository::create_repo(".").unwrap(),
        },
    };
}
