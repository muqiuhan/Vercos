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

pub mod create;
mod path;

use crate::error::{self, Log};
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
            error::repo::Repo::NotLitRepo(lit_dir.clone()).panic();
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

    pub fn new_with_pathbuf(path: &Path, force: bool) -> Self {
        Self::new(&path.to_str().unwrap().to_string(), force)
    }

    pub(self) fn check_repositoryformatversion(conf: &mut Ini) -> error::Result<()> {
        let repositoryformatversion = conf
            .with_section(Some("core"))
            .get("repositoryformatversion")
            .unwrap()
            .to_string();

        if repositoryformatversion != "0" {
            error::repo::Repo::UnsupportedRepositoryFormatVersion(repositoryformatversion).panic()
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
                    error::repo::Repo::MissingConfigFile(path).panic();
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
