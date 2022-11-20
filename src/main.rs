use clap::{Parser, Subcommand, ValueEnum};

mod object;
mod repository;
mod utils;
use object::read_object;
use repository::Repository;
use utils::to_hex_string;

use crate::object::find_object;

#[derive(Parser)]
#[command(name = "mog")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<String> },
    CatFile { object_type: Type, object: String },
}

#[derive(Clone, ValueEnum)]
enum Type {
    Blob,
    Tree,
    Commit,
    Tag,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init { path } => {
            match path {
                Some(path) => Repository::create_repo(&path).unwrap(),
                None => Repository::create_repo(".").unwrap(),
            };
            ()
        }
        Commands::CatFile {
            object_type,
            object,
        } => {
            let repo = Repository::find_repo(".").unwrap();
            let object_type = match object_type {
                Type::Blob => "blob",
                Type::Tree => "tree",
                Type::Commit => "commit",
                Type::Tag => "tag",
            };
            let obj = read_object(
                &repo,
                &find_object(&repo, &object, object_type, true).unwrap(),
            )
            .unwrap();
            println!("{}", to_hex_string(&obj.serialize().unwrap()));
        }
    };
}
