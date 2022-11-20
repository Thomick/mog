use configparser::ini::Ini;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    pub conf: Ini,
}

impl Repository {
    pub fn new(path: &str, force: bool) -> Result<Repository, String> {
        let worktree = Path::new(path).to_path_buf();
        let gitdir = worktree.join(".git");
        let mut conf = Ini::new();

        if !force && !gitdir.exists() {
            return Err(format!("Not a  repository {}", path));
        }

        // Read configuration file in .git/config
        let cf = gitdir.join("config");
        if cf.exists() {
            match conf.load(cf.to_str().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(format!("Error reading configuration file: {}", e)),
            };
        } else if !force {
            return Err(format!("Configuration file missing"));
        }

        if !force {
            let vers = conf.getint("core", "repositoryformatversion")?.unwrap();
            if vers != 0 {
                return Err(format!("Unsupported repositoryformatversion {}", vers));
            }
        }

        Ok(Repository {
            worktree: worktree,
            gitdir: gitdir,
            conf: conf,
        })
    }

    pub fn create_repo(path: &str) -> Result<Repository, String> {
        let mut repo = Repository::new(path, true)?;

        if repo.gitdir.exists() {
            return Err(format!("repository already exists"));
        }

        // Create .git directory
        match std::fs::create_dir_all(&(repo.gitdir)) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Error creating repository directory ({}): {}",
                    repo.gitdir.to_str().unwrap(),
                    e
                ))
            }
        };

        // Create initial config file
        let cf = repo.gitdir.join("config");
        repo.conf
            .set("core", "repositoryformatversion", Some("0".to_string()));
        repo.conf.set("core", "filemode", Some("false".to_string()));
        repo.conf.set("core", "bare", Some("false".to_string()));
        match repo.conf.write(cf.to_str().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Error writing configuration file ({}): {}",
                    cf.to_str().unwrap(),
                    e
                ))
            }
        };

        Ok(repo)
    }

    fn find_repo(path: &str) -> Result<Repository, String> {
        let path = Path::new(path);

        if path.join(".git").exists() {
            return Repository::new(path.to_str().unwrap(), false);
        }
        let parent = path.parent();
        match parent {
            Some(parent) => Repository::find_repo(parent.to_str().unwrap()),
            None => Err(format!("Not in a git repository")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[test]
    fn test_create_repo() {
        let path = "test_create_repo";
        let _ = remove_dir_all(path);
        let repo = Repository::create_repo(path).unwrap();
        assert_eq!(repo.worktree, Path::new(path).to_path_buf());
        assert_eq!(
            repo.gitdir,
            Path::new(&format!("{}/.git", path)).to_path_buf()
        );
        assert_eq!(
            repo.conf.get("core", "repositoryformatversion"),
            Some("0".to_string())
        );
        assert_eq!(repo.conf.get("core", "filemode"), Some("false".to_string()));
        assert_eq!(repo.conf.get("core", "bare"), Some("false".to_string()));
        assert!(repo.gitdir.exists());
        let _ = remove_dir_all(path);
    }

    #[test]
    fn test_find_repo() {
        let path = "test_find_repo";
        let _ = remove_dir_all(path);
        let repo = Repository::create_repo(path).unwrap();
        assert!(repo.gitdir.exists());
        let repo2 = Repository::find_repo(path).unwrap();
        assert_eq!(repo.worktree, repo2.worktree);
        assert_eq!(repo.gitdir, repo2.gitdir);
        assert_eq!(repo.conf, repo2.conf);
        let _ = remove_dir_all(path);
    }

    #[test]
    fn test_find_repo_deep() {
        let path = "test_find_repo_deep";
        let _ = remove_dir_all(path);
        let repo = Repository::create_repo(path).unwrap();
        let repo2 = Repository::find_repo(&format!("{}/test", path)).unwrap();
        assert_eq!(repo.worktree, repo2.worktree);
        assert_eq!(repo.gitdir, repo2.gitdir);
        assert_eq!(repo.conf, repo2.conf);
        let _ = remove_dir_all(path);
    }
}
