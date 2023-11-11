use crate::commands::cat_file::CatFile;
use crate::error;
use crate::error::Log;
use crate::object::{blob, Object};
use crate::repo::Repo;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::io::Read;

/// Read object sha from lit repository repo.
/// Return a Object whose exact type depends on the object.
pub fn read(repo: &Repo, sha: &str) -> Option<Box<dyn Object>> {
    let path = Repo::repo_file(&repo.lit_dir, &["objects", &sha[0..2], &sha[2..]], false).unwrap();

    if !(path.is_file()) {
        None
    } else {
        let mut raw = Vec::new();
        ZlibDecoder::new(std::fs::read(path).unwrap().as_slice())
            .read_to_end(&mut raw)
            .unwrap();

        // Read the object type
        let x = raw.iter().position(|byte| *byte == 0x20u8).unwrap();
        let fmt = std::str::from_utf8(&raw[0..x]).unwrap();

        // Read and validate object size
        let y = raw[x..].iter().position(|byte| *byte == 0x00u8).unwrap();
        let size = std::str::from_utf8(&raw[x + 1..x + y])
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // X itself occupied one bit
        if size != raw.len() - (x + y + 1) {
            error::object::Object::Malformed(sha.to_owned(), size).panic();
        } else {
            match fmt {
                // "commit" => Commit(&raw[y + 1..]),
                // "tree" => Tree(&raw[y + 1..]),
                // "tag" => Tag(&raw[y + 1..]),
                "blob" => Some(Box::new(blob::Blob::deserialize(
                    std::str::from_utf8(&raw[x + y + 1..]).unwrap(),
                ))),
                typ => error::object::Object::UnknownType(typ.to_string(), sha.to_owned()).panic(),
            }
        }
    }
}

pub fn write(object: Box<dyn Object>, repo: Option<Repo>) -> String {
    let data = object.serialize();

    let result = format!(
        "{} {}\0{}",
        object.fmt(),
        &data.len(),
        std::str::from_utf8(data).unwrap()
    );

    let sha = {
        let mut hasher = Sha1::new();
        hasher.update(&result);
        format!("{:x}", hasher.finalize())
    };

    repo.iter().for_each(|repo| {
        let path =
            Repo::repo_file(&repo.lit_dir, &["objects", &sha[0..2], &sha[2..]], true).unwrap();

        if !(path.exists()) {
            fs::write(path, {
                let mut compress = ZlibEncoder::new(Vec::new(), Compression::default());

                compress.write_all(result.as_bytes()).unwrap();
                compress.finish().unwrap()
            })
            .unwrap();
        }
    });

    sha.to_string()
}

/// The reason for this strange small function is that
/// lit has a lot of ways to refer to objects: full hash, short hash, tags...
/// This function is the name resolution function.
fn find(_repo: &Repo, name: &str, _fmt: &str, _follow: bool) -> String {
    name.to_owned()
}

pub fn cat(args: &CatFile) -> String {
    let repo = Repo::repo_find(&".".to_owned(), true).unwrap();
    let object = read(&repo, find(&repo, &args.object, &args.typ, true).as_str()).unwrap();
    std::str::from_utf8(object.serialize()).unwrap().to_string()
}

/// Hash object, writing it to repo if provided
pub fn hash(file: &String, fmt: &str, repo: Option<Repo>) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let object = match fmt {
        // "commit" => Commit(&raw[y + 1..]),
        // "tree" => Tree(&raw[y + 1..]),
        // "tag" => Tag(&raw[y + 1..]),
        "blob" => Box::new(blob::Blob::new(data)),
        typ => error::object::Object::UnknownType(typ.to_string(), file.to_owned()).panic(),
    };

    write(object, repo)
}

#[cfg(test)]
mod test {
    use crate::commands::cat_file::CatFile;
    use crate::commands::init::Init;
    use crate::object::blob::Blob;
    use crate::object::operation::write;
    use crate::repo;

    use flate2::bufread::ZlibDecoder;
    use std::{fs, io::Read, path::PathBuf};

    use super::cat;

    #[test]
    pub fn test_read_blob() {
        let path = PathBuf::from("./assests/blob_object");
        let raw = {
            let mut vec = Vec::new();
            ZlibDecoder::new(std::fs::read(path).unwrap().as_slice())
                .read_to_end(&mut vec)
                .unwrap();

            vec
        };

        let x = raw.iter().position(|byte| *byte == 0x20u8).unwrap();
        let fmt = std::str::from_utf8(&raw[0..x]).unwrap();

        let y = &raw[x..].iter().position(|byte| *byte == 0x00).unwrap();
        let size = std::str::from_utf8(&raw[x + 1..x + y])
            .unwrap()
            .parse::<usize>()
            .unwrap();

        assert_eq!(x, 4);
        assert_eq!(*y, 5);
        assert_eq!(fmt, "blob");
        assert_eq!(size, 1505);
    }

    #[test]
    pub fn test_write_blob() {
        // Prevent directory from being deleted
        let _ = fs::remove_dir_all(".lit");
        let repo = repo::Repo::create(&Init {
            force: false,
            path: String::from("."),
        });

        let object = Blob::new("Ok, this is a blob object".to_string());
        let sha1 = write(Box::new(object), Some(repo));

        assert_eq!("9ca6e1d93dfc2343e4e404a6b742220b148649a0", sha1.as_str());
        assert!(PathBuf::from(".lit/objects/9c/a6e1d93dfc2343e4e404a6b742220b148649a0").exists());
        fs::remove_dir_all(".lit").unwrap();
    }

    #[test]
    pub fn test_cat_file() {
        // Prevent directory from being deleted
        let _ = fs::remove_dir_all(".lit");

        let repo = repo::Repo::create(&Init {
            force: false,
            path: String::from("."),
        });

        let object = Blob::new("Ok, this is a blob object".to_string());
        let sha1 = write(Box::new(object), Some(repo));

        let content = cat(&CatFile {
            typ: "blob".to_string(),
            object: sha1,
        });

        fs::remove_dir_all(".lit").unwrap();
        assert_eq!("Ok, this is a blob object".to_string(), content);
    }
}
