use crate::repository::Repository;
use crate::utils::to_hex_string;
use flate2::read::ZlibDecoder;
use sha1::Digest;
use sha1::Sha1;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub trait Object {
    fn serialize(&self) -> Result<Vec<u8>, String> {
        Err("Not implemented".to_string())
    }

    fn deserialize(data: &[u8]) -> Result<Box<dyn Object>, String>
    where
        Self: Sized,
    {
        Err("Not implemented".to_string())
    }

    fn get_type(&self) -> String;
}

pub struct Blob {
    content: Vec<u8>,
}

impl Object for Blob {
    fn serialize(&self) -> Result<Vec<u8>, String> {
        Ok(self.content.clone())
    }

    fn deserialize(data: &[u8]) -> Result<Box<dyn Object>, String> {
        Ok(Box::new(Blob {
            content: data.to_vec(),
        }))
    }

    fn get_type(&self) -> String {
        "blob".to_string()
    }
}

impl Blob {
    fn from_object(obj: &dyn Object) -> Result<Blob, String> {
        match obj.get_type().as_str() {
            "blob" => Ok(Blob {
                content: obj.serialize().unwrap(),
            }),
            _ => Err(format!("Casting error : Not a blob")),
        }
    }

    pub fn new(content: Vec<u8>) -> Blob {
        Blob { content }
    }
}

pub struct Tree;

impl Object for Tree {
    fn get_type(&self) -> String {
        "tree".to_string()
    }
}

impl Tree {
    pub fn new() -> Tree {
        panic!("Not implemented");
        Tree {}
    }
}

pub struct Commit;

impl Object for Commit {
    fn get_type(&self) -> String {
        "commit".to_string()
    }
}

impl Commit {
    pub fn new() -> Commit {
        panic!("Not implemented");
        Commit {}
    }
}

pub struct Tag;

impl Object for Tag {
    fn get_type(&self) -> String {
        "tag".to_string()
    }
}

impl Tag {
    pub fn new() -> Tag {
        panic!("Not implemented");
        Tag {}
    }
}

pub fn read_object(repo: &Repository, sha: &str) -> Result<Box<dyn Object>, String> {
    let path = repo.gitdir.join("objects").join(&sha[0..2]).join(&sha[2..]);
    let f = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!(
                "Error opening object({}):{}",
                path.to_str().unwrap(),
                e
            ))
        }
    };
    let mut decoder = ZlibDecoder::new(f);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data).unwrap();

    // Read object type
    let x = data.iter().position(|&r| r == b' ').unwrap();
    let obj_type = String::from_utf8(data[0..x].to_vec()).unwrap();

    // Read object size
    let y = data.iter().position(|&r| r == b'\0').unwrap();
    let obj_size: usize = String::from_utf8(data[x + 1..y].to_vec())
        .unwrap()
        .parse()
        .unwrap();

    // Read object data
    let obj_data = &data[y + 1..];
    assert!(obj_data.len() == obj_size);

    match obj_type.as_str() {
        "blob" => Ok(Blob::deserialize(obj_data).unwrap()),
        "tree" => Ok(Tree::deserialize(obj_data).unwrap()),
        "commit" => Ok(Commit::deserialize(obj_data).unwrap()),
        "tag" => Ok(Tag::deserialize(obj_data).unwrap()),
        _ => Err(format!("Unknown object type {}", obj_type)),
    }
}

pub fn write_object(
    repo: &Repository,
    obj: &dyn Object,
    actually_write: bool,
) -> Result<String, String> {
    let data = obj.serialize()?;
    let obj_size = data.len();
    let mut sha = Sha1::new();
    let header = format!("{} {}\0", obj.get_type(), obj_size);
    sha.update(header.as_bytes());
    sha.update(&data);
    let sha = to_hex_string(&sha.finalize());
    if !actually_write {
        return Ok(sha);
    }

    let path = repo.gitdir.join("objects").join(&sha[0..2]).join(&sha[2..]);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let f = File::create(path).unwrap();
    let mut encoder = flate2::write::ZlibEncoder::new(f, flate2::Compression::default());
    encoder.write_all(header.as_bytes()).unwrap();
    encoder.write_all(&data).unwrap();
    encoder.finish().unwrap();

    Ok(sha)
}

pub fn find_object(
    repo: &Repository,
    name: &str,
    obj_type: &str,
    follow: bool,
) -> Result<String, String> {
    let path = repo
        .gitdir
        .join("objects")
        .join(&name[0..2])
        .join(&name[2..]);
    if path.exists() {
        Ok(name.to_string())
    } else {
        Err(format!("Object not found: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[test]
    fn test_write_blob() {
        let path = "test_write_blob";
        let _ = remove_dir_all(path);
        let repo = Repository::create_repo(path).unwrap();
        let blob = Blob {
            content: "Hello, world!".as_bytes().to_vec(),
        };
        let sha = write_object(&repo, &blob, true).unwrap();
        assert_eq!(sha, "e290f1a2f1d404309f6d614728256282519b50b6");
        assert!(repo
            .gitdir
            .join("objects")
            .join("e2")
            .join("90f1a2f1d404309f6d614728256282519b50b6")
            .exists());
        //let _ = remove_dir_all(path);
    }

    #[test]
    fn test_read_blob() {
        let path = "test_read_blob";
        let _ = remove_dir_all(path);
        let repo = Repository::create_repo(path).unwrap();
        let blob = Blob {
            content: "Hello, world!".as_bytes().to_vec(),
        };
        let sha = write_object(&repo, &blob, true).unwrap();
        let obj = Blob::from_object(read_object(&repo, &sha).unwrap().as_ref()).unwrap();
        assert_eq!(obj.get_type(), "blob");
        assert_eq!(obj.content, "Hello, world!".as_bytes().to_vec());
        let _ = remove_dir_all(path);
    }
}
