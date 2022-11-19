use clap::Parser;
use clap::Subcommand;

mod object;
mod repository;
mod utils;
use repository::GitRepository;

#[derive(Parser)]
#[command(name = "tvcs")]
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
            Some(path) => GitRepository::create_repo(&path).unwrap(),
            None => GitRepository::create_repo(".").unwrap(),
        },
    };
}
