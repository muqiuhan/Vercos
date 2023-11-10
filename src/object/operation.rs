
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
use crate::commands::cat_file::CatFile;

/// Read object sha from lit repository repo.
/// Return a Object whose exact type depends on the object.
pub fn read(repo: &Repo, sha: &str) -> Option<Box<dyn Object>> {
    let path = Repo::repo_file(&repo.lit_dir, &["objects", &sha[0..2], &sha[2..]], false).unwrap();

    if !(path.is_file()) {
        None
    } else {
        let mut raw = String::new();
        ZlibDecoder::new(std::fs::read(path).unwrap().as_slice())
            .read_to_string(&mut raw)
            .unwrap();

        // Read the object type
        let x = raw.find(' ').unwrap();
        let fmt = &raw[0..x];

        // Read and validate object size
        let y = raw[x..].find('\x00').unwrap();
        let size = raw[x..y].parse::<usize>().unwrap();

        if size != raw.len() - y - 1 {
            error::object::Object::Malformed(sha.to_owned(), size).panic();
        } else {
            match fmt {
                // "commit" => Commit(&raw[y + 1..]),
                // "tree" => Tree(&raw[y + 1..]),
                // "tag" => Tag(&raw[y + 1..]),
                "blob" => Some(Box::new(blob::Blob::deserialize(&raw[y + 1..]))),
                typ => error::object::Object::UnknownType(typ.to_string(), sha.to_owned()).panic(),
            }
        }
    }
}

pub fn write(object: &dyn Object, repo: Option<Repo>) -> String {
    let data = object.serialize();
    let result = format!(
        "{}  {}\x00{}",
        object.fmt(),
        &data.len(),
        std::str::from_utf8(data).unwrap()
    );

    let sha = {
        let mut hasher = Sha1::new();
        hasher.update(result.as_bytes());
        std::str::from_utf8(&hasher.finalize()).unwrap().to_string()
    };

    repo.iter().for_each(|repo| {
        let path =
            Repo::repo_file(&repo.lit_dir, &["objects", &sha[0..2], &sha[2..]], true).unwrap();

        if !(path.exists()) {
            fs::write(path, {
                let mut compress = ZlibEncoder::new(Vec::new(), Compression::default());

                compress.write_all(sha.as_bytes()).unwrap();
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
fn object_find(_repo: &Repo, name: &String, _fmt: &String, _follow: bool) -> String {
    name.clone()
}

pub fn cat(args: &CatFile) {
    let repo = Repo::repo_find(&".".to_owned(), true).unwrap();
    let _object = read(
        &repo,
        object_find(&repo, &args.object, &args.typ, true).as_str(),
    );
}

#[cfg(test)]
mod test {
    use flate2::bufread::ZlibDecoder;
    use std::{io::Read, path::PathBuf};

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
    pub fn test_write_blob() {}
}
