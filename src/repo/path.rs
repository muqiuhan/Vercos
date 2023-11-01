use crate::error::{self, Error, Repo};
use std::{fs, path::PathBuf};

impl crate::repo::Repo {
    /// Compute path under repo's litdir
    pub fn repo_path(lit_dir: &PathBuf, path: &[&str]) -> error::Result<PathBuf> {
        Ok(path
            .iter()
            .fold(lit_dir.to_owned(), |repo_path, path| repo_path.join(path)))
    }

    /// Same as repo_path, but create dirname(*path) if absent.
    /// For example, repo_file(r, \"refs\", \"remotes\", \"origin\", \"HEAD\") will create .lit/refs/remotes/origin
    pub fn repo_file(lit_dir: &PathBuf, path: &[&str], mkdir: bool) -> error::Result<PathBuf> {
        Self::repo_dir(lit_dir, &path[0..path.len() - 1], mkdir)
    }

    /// Same as repo_path, but mkdir *path if absent if mkdir.
    pub fn repo_dir(lit_dir: &PathBuf, path: &[&str], mkdir: bool) -> error::Result<PathBuf> {
        let path = Self::repo_path(lit_dir, path)?;

        if path.exists() {
            if path.is_dir() {
                Ok(path)
            } else {
                Err(Error::Repo(Repo::NotDirectory(path)))
            }
        } else if mkdir {
            fs::create_dir_all(&path).unwrap();
            return Ok(path);
        } else {
            panic!("repo_dir({:?}, {:?})", lit_dir, path)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::repo::Repo;
    use std::path::PathBuf;

    #[test]
    pub fn test_repo_path() {
        let lit_dir = PathBuf::from(".lit");
        let path = ["a", "b", "c"];
        assert_eq!(
            PathBuf::from(".lit").join("a").join("b").join("c"),
            Repo::repo_path(&lit_dir, &path).unwrap()
        )
    }
}
