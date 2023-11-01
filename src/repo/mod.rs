mod path;

use crate::error::{self, Error};
use crate::r#const::LIT_DIR;
use ini::Ini;
use std::path::{Path, PathBuf};

/// The lit repository
pub struct Repo {
    worktree: String,
    lit_dir: String,
    conf: Ini,
}

impl Repo {
    pub fn new(path: &String, force: bool) -> Self {
        let worktree = path.clone();
        let lit_dir = Path::new(path).join(LIT_DIR);

        if !(force || Path::new(&lit_dir).is_dir()) {
            Error::Repo(error::Repo::NotLitRepo(lit_dir.clone())).panic();
        }

        let mut conf = Self::read_conf_file(&lit_dir, force).unwrap();
        Self::check_repositoryformatversion(&mut conf).unwrap();

        Repo {
            worktree,
            lit_dir: String::from(lit_dir.to_str().unwrap()),
            conf,
        }
    }

    pub(self) fn check_repositoryformatversion(conf: &mut Ini) -> error::Result<()> {
        let repositoryformatversion = conf
            .with_section(Some("core"))
            .get("repositoryformatversion")
            .unwrap()
            .to_string();

        if repositoryformatversion != "0" {
            Err(Error::Repo(
                error::Repo::UnsupportedRepositoryFormatVersion(repositoryformatversion),
            ))
        } else {
            Ok(())
        }
    }

    pub(self) fn read_conf_file(lit_dir: &PathBuf, force: bool) -> error::Result<Ini> {
        let conf = Self::repo_file(lit_dir, &vec!["conf"], false)?;

        if conf.exists() {
            Ok(Ini::load_from_file(conf).unwrap())
        } else if !force {
            Err(Error::Repo(error::Repo::MissingConfigFile(conf)))
        } else {
            unimplemented!()
        }
    }
}
