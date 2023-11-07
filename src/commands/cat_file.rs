use std::path::PathBuf;

use crate::{object, repo::Repo};

/// Provide content of repository objects
pub struct CatFile {
    /// Specify the type (blob | commit | tag | tree)
    pub typ: String,

    /// The object to display
    pub object: String,
}

impl CatFile {
    pub fn cat(&self) {
        let repo = Repo::repo_find(&".".to_owned(), true).unwrap();
        let object = object::read(
            &repo,
            Self::object_find(&repo, &self.object, &self.typ, true).as_str(),
        );
    }

    /// The reason for this strange small function is that
    /// lit has a lot of ways to refer to objects: full hash, short hash, tags...
    /// This function is the name resolution function.
    fn object_find(repo: &Repo, name: &String, fmt: &String, follow: bool) -> String {
        name.clone()
    }
}
