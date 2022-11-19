use crate::repository::GitRepository;
use crate::utils::to_hex_string;
use flate2::read::ZlibDecoder;
use sha1::Digest;
use sha1::Sha1;
use std::fs::File;
use std::io::Read;
use std::io::Write;

trait GitObject {
    fn from(&self, repo: &GitRepository, data: &[u8]) -> Result<Box<dyn GitObject>, String>;
    fn serialize(&self) -> Result<Vec<u8>, String>;
    fn deserialize(&self, data: &[u8]) -> Result<Box<dyn GitObject>, String>;
    fn get_type(&self) -> String;
}

enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

fn read_object(repo: &GitRepository, sha: &str) -> Result<ObjectType, String> {
    let path = repo.gitdir.join("objects").join(&sha[0..2]).join(&sha[2..]);
    let f = File::open(path).unwrap();
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
    let obj_data = data[y + 1..].to_vec();
    assert!(obj_data.len() == obj_size);

    match obj_type.as_str() {
        "blob" => Ok(ObjectType::Blob),
        "tree" => Ok(ObjectType::Tree),
        "commit" => Ok(ObjectType::Commit),
        "tag" => Ok(ObjectType::Tag),
        _ => Err(format!("Unknown object type {}", obj_type)),
    }
}

fn write_object(repo: &GitRepository, obj: &dyn GitObject) -> Result<String, String> {
    let data = obj.serialize()?;
    let obj_size = data.len();
    let mut sha = Sha1::new();
    let result = format!("{} {}\0", obj.get_type(), obj_size);
    sha.update(result.as_bytes());
    let sha = sha.finalize();

    let path = repo
        .gitdir
        .join("objects")
        .join(to_hex_string(&sha[0..2]))
        .join(to_hex_string(&sha[2..]));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let f = File::create(path).unwrap();
    let mut encoder = flate2::write::ZlibEncoder::new(f, flate2::Compression::default());
    encoder
        .write_all(format!("blob {}\0", obj_size).as_bytes())
        .unwrap();
    encoder.write_all(&data).unwrap();
    encoder.finish().unwrap();

    Ok(to_hex_string(&sha))
}
