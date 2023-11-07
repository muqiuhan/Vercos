use crate::object::Object;
use crate::repo::Repo;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;

pub fn write<T>(object: &Box<dyn Object>, repo: Option<Repo>) -> String {
    let data = object.serialize();
    let result = format!(
        "{}  {}\x00{}",
        object.fmt(),
        &data.len(),
        std::str::from_utf8(data).unwrap()
    );

    let sha: &str = {
        let mut hasher = Sha1::new();
        hasher.update(result.as_bytes());
        std::str::from_utf8(&hasher.finalize()).unwrap()
    };

    repo.iter().for_each(|repo| {
        let path =
            Repo::repo_file(&repo.lit_dir, &["objects", &sha[0..2], &sha[2..]], true).unwrap();

        if !(path.exists()) {
            fs::write(path, {
                let mut compress = ZlibEncoder::new(Vec::new(), Compression::default());

                compress.write_all(&sha.as_bytes()).unwrap();
                compress.finish().unwrap()
            })
            .unwrap();
        }
    });

    sha.to_string()
}
