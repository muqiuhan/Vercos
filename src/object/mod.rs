/*
 * Copyright (C) 2023 Muqiu Han
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

mod blob;

use crate::error;
use crate::error::Log;
use crate::repo::Repo;
use flate2::read::ZlibDecoder;
use std::io::Read;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;

pub trait Object {
    fn to_string(&self) -> String;
    fn deserialize(data: &str) -> Self
    where
        Self: Sized;

    /// It must read the object's contents from data, a byte string, and do
    /// whatever it takes to convert it into a meaningful representation.
    fn serialize(&self) -> &[u8];

    fn fmt(&self) -> &String;
}

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

#[cfg(test)]
mod test {
    use flate2::bufread::ZlibDecoder;
    use std::{io::Read, path::PathBuf};

    #[test]
    pub fn test_read() {
        let path = PathBuf::from(".git/objects/1d/b99e919e2087dad777c3e088bf97deef862666");
        let raw = {
            let mut str = String::new();
            ZlibDecoder::new(std::fs::read(path).unwrap().as_slice())
                .read_to_string(&mut str)
                .unwrap();
            str
        };

        // Read the object type
        let x = raw.find(' ').unwrap();
        let _fmt = &raw[0..x];

        // Read and validate object size
        let y = raw[x..].find('\x00').unwrap();
        let _size = &raw[x..y].to_string();
    }
}
