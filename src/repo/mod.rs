/// Copyright (C) 2023 Muqiu Han
mod path;

use crate::error::{self, Error, Log};
use crate::r#const::LIT_DIR;
use ini::Ini;
use std::path::{Path, PathBuf};

/// The lit repository
pub struct Repo {
    pub worktree: PathBuf,
    pub lit_dir: PathBuf,
    pub conf: Option<Ini>,
}

impl Repo {
    pub fn new(path: &String, force: bool) -> Self {
        let worktree = PathBuf::from(path);
        let lit_dir = Path::new(path).join(LIT_DIR);

        if !(force || Path::new(&lit_dir).is_dir()) {
            error::Repo::NotLitRepo(lit_dir.clone()).panic();
        }

        match Self::read_conf_file(&lit_dir, force) {
            Some(mut conf) => {
                Self::check_repositoryformatversion(&mut conf).unwrap();
                Repo {
                    worktree,
                    lit_dir,
                    conf: Some(conf),
                }
            }
            None => Repo {
                worktree,
                lit_dir,
                conf: None,
            },
        }
    }

    pub fn new_with_pathbuf(path: &PathBuf, force: bool) -> Self {
        Self::new(&path.to_str().unwrap().to_string(), force)
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

    pub(self) fn read_conf_file(lit_dir: &PathBuf, force: bool) -> Option<Ini> {
        let conf = Self::repo_file(lit_dir, &["config"], false);

        match conf {
            Some(path) => {
                if path.exists() {
                    Some(Ini::load_from_file(path).unwrap())
                } else if !force {
                    error::Repo::MissingConfigFile(path).panic();
                } else if force {
                    Some(Ini::load_from_file(path).unwrap())
                } else {
                    panic!("read_conf_file({:?}, {:?})", lit_dir, force)
                }
            }
            _ => None,
        }
    }
}
