use crate::error::{Error, Repo};
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
                Error::Repo(Repo::NotDirectory(path)).panic();
            }
        } else if mkdir {
            fs::create_dir_all(&path).unwrap();
            return Some(path);
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
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
        let lit_dir = PathBuf::from(".lit");
        let expect = PathBuf::from(".lit")
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
