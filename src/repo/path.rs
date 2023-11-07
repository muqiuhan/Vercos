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

use crate::error;
use crate::error::Log;
use crate::r#const::LIT_DIR;
use std::{fs, path::PathBuf};

impl crate::repo::Repo {
    /// Compute path under repo's lit_dir
    pub fn repo_path(lit_dir: &PathBuf, path: &[&str]) -> Option<PathBuf> {
        Some(
            path.iter()
                .fold(lit_dir.to_owned(), |repo_path, path| repo_path.join(path)),
        )
    }

    /// Same as repo_path, but create directory if absent.
    /// For example, `repo_file(".lit", ["refs", "remotes", "origin", "HEAD"])`
    /// will create .lit/refs/remotes/origin
    pub fn repo_file(lit_dir: &PathBuf, path: &[&str], mkdir: bool) -> Option<PathBuf> {
        match Self::repo_dir(lit_dir, &path[0..path.len() - 1], mkdir) {
            Some(_) => Self::repo_path(lit_dir, path),
            None => None,
        }
    }

    /// Same as repo_path, but mkdir *path if absent if mkdir.
    pub fn repo_dir(lit_dir: &PathBuf, path: &[&str], mkdir: bool) -> Option<PathBuf> {
        let path = Self::repo_path(lit_dir, path)?;

        if path.exists() {
            if path.is_dir() {
                Some(path)
            } else {
                error::repo::Repo::NotDirectory(path).panic()
            }
        } else if mkdir {
            fs::create_dir_all(&path).unwrap();
            return Some(path);
        } else {
            None
        }
    }

    /// Look for that root, starting at the current directory and recursing back to /.
    /// To identify a path as a repository, it will check for the presence of a `LIT_DIR` directory.
    pub fn repo_find(path: &String, required: bool) -> Option<Self> {
        let path = std::fs::canonicalize(PathBuf::from(path)).unwrap();

        if path.join(LIT_DIR).is_dir() {
            Some(Self::new_with_pathbuf(&path, false))
        } else {
            // Recurse in parent
            let parent = std::fs::canonicalize(PathBuf::from(&path).join("..")).unwrap();

            // At root directory (`/.. == /`)
            if parent.eq(&path) {
                if required {
                    error::repo::Repo::CannotFindLitRepo(path).panic();
                } else {
                    None
                }
            } else {
                Self::repo_find(&parent.to_str().unwrap().to_string(), required)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r#const::LIT_DIR;
    use crate::repo::Repo;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    pub fn test_repo_path() {
        let lit_dir = PathBuf::from(".lit");
        let expect = PathBuf::from(".lit").join("a").join("b").join("c");

        let path = ["a", "b", "c"];
        let path = Repo::repo_path(&lit_dir, &path).unwrap();

        assert_eq!(expect, path)
    }

    #[test]
    pub fn test_repo_file() {
        let lit_dir = PathBuf::from(".lit");
        let expect = PathBuf::from(".lit")
            .join("refs")
            .join("remotes")
            .join("origin")
            .join("HEAD");

        fs::create_dir_all(&expect).unwrap();

        let path = ["refs", "remotes", "origin", "HEAD"];
        let path = Repo::repo_file(&lit_dir, &path, false).unwrap();

        assert_eq!(expect, path);

        fs::remove_dir_all(".lit").unwrap();
    }

    #[test]
    pub fn test_repo_file_with_mkdir() {
        let lit_dir = PathBuf::from(".lit");
        let expect = PathBuf::from(".lit")
            .join("refs")
            .join("remotes")
            .join("origin")
            .join("HEAD");

        let path = ["refs", "remotes", "origin", "HEAD"];
        let path = Repo::repo_file(&lit_dir, &path, true).unwrap();

        assert_eq!(expect, path);

        fs::remove_dir_all(".lit").unwrap();
    }

    #[test]
    pub fn test_repo_dir() {
        let lit_dir = PathBuf::from(LIT_DIR);
        let expect = PathBuf::from(LIT_DIR)
            .join("refs")
            .join("remotes")
            .join("origin")
            .join("HEAD");

        fs::create_dir_all(&expect).unwrap();

        let path = ["refs", "remotes", "origin", "HEAD"];
        let path = Repo::repo_dir(&lit_dir, &path, false).unwrap();

        assert_eq!(expect, path);

        fs::remove_dir_all(".lit").unwrap();
    }

    #[test]
    pub fn test_repo_dir_with_mkdir() {
        let lit_dir = PathBuf::from(".lit");
        let expect = PathBuf::from(".lit")
            .join("refs")
            .join("remotes")
            .join("origin")
            .join("HEAD");
        let path = ["refs", "remotes", "origin", "HEAD"];
        let path = Repo::repo_dir(&lit_dir, &path, true).unwrap();

        assert_eq!(expect, path);

        fs::remove_dir_all(".lit").unwrap();
    }
}
