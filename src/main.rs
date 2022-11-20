use clap::{Parser, Subcommand, ValueEnum};

mod object;
mod repository;
mod utils;
use object::{read_object, write_object};
use object::{Blob, Commit, Object, Tag, Tree};
use repository::Repository;
use std::fs::read;
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
    Init {
        path: Option<String>,
    },
    CatFile {
        object_type: ObjectType,
        object: String,
    },
    HashObject {
        object_type: ObjectType,
        file: String,
        #[arg(short, long)]
        write: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum ObjectType {
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
                ObjectType::Blob => "blob",
                ObjectType::Tree => "tree",
                ObjectType::Commit => "commit",
                ObjectType::Tag => "tag",
            };
            let obj = read_object(
                &repo,
                &find_object(&repo, &object, object_type, true).unwrap(),
            )
            .unwrap();
            println!("{}", String::from_utf8(obj.serialize().unwrap()).unwrap());
        }
        Commands::HashObject {
            object_type,
            file,
            write,
        } => {
            let repo = Repository::find_repo(".").unwrap();
            let data = read(file).unwrap();
            let obj: Box<dyn Object> = match object_type {
                ObjectType::Blob => Box::new(Blob::new(data)),
                ObjectType::Tree => Box::new(Tree::new()),
                ObjectType::Commit => Box::new(Commit::new()),
                ObjectType::Tag => Box::new(Tag::new()),
            };
            println!("{}", write_object(&repo, obj.as_ref(), write).unwrap());
        }
    };
}
