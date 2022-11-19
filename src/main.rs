use clap::Parser;
use clap::Subcommand;
use configparser::ini::Ini;
use std::path::{Path, PathBuf};

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

struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: Ini,
}

impl GitRepository {
    fn new(path: &str, force: bool) -> Result<GitRepository, String> {
        let worktree = Path::new(path).to_path_buf();
        let gitdir = worktree.join(".git");
        let mut conf = Ini::new();

        if !force && !gitdir.exists() {
            return Err(format!("Not a Git repository {}", path));
        }

        // Read configuration file in .git/config
        let cf = gitdir.join("config");
        if cf.exists() {
            conf.load(cf.to_str().unwrap()).unwrap();
        } else if !force {
            return Err(format!("Configuration file missing"));
        }

        if !force {
            let vers = conf.getint("core", "repositoryformatversion")?.unwrap();
            if vers != 0 {
                return Err(format!("Unsupported repositoryformatversion {}", vers));
            }
        }

        Ok(GitRepository {
            worktree: worktree,
            gitdir: gitdir,
            conf: conf,
        })
    }

    fn create_repo(path: &str) -> Result<GitRepository, String> {
        let mut repo = GitRepository::new(path, true)?;

        if repo.gitdir.exists() {
            return Err(format!("Git repository already exists"));
        }

        // Create .git directory
        std::fs::create_dir_all(&(repo.gitdir)).unwrap();

        // Create initial config file
        let cf = repo.gitdir.join("config");
        repo.conf
            .set("core", "repositoryformatversion", Some("0".to_string()));
        repo.conf.set("core", "filemode", Some("false".to_string()));
        repo.conf.set("core", "bare", Some("false".to_string()));
        repo.conf.write(cf.to_str().unwrap()).unwrap();

        Ok(repo)
    }

    fn find_repo(path: &str) -> Result<GitRepository, String> {
        let path = Path::new(path);

        if path.join(".git").exists() {
            return GitRepository::new(path.to_str().unwrap(), false);
        }
        let parent = path.parent();
        match parent {
            Some(parent) => GitRepository::find_repo(parent.to_str().unwrap()),
            None => Err(format!("Not in a git repository")),
        }
    }
}
