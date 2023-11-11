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

use crate::error::Log;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Repo {
    NotLitRepo(PathBuf),
    CannotFindLitRepo,
    NotDirectory(PathBuf),
    NotEmpty(PathBuf),
    MissingConfigFile(PathBuf),
    UnsupportedRepositoryFormatVersion(String),
}

impl Log for Repo {
    fn fmt(&self) -> String {
        match self {
            Repo::NotLitRepo(dir) => format!("`{}` is not a lit repository", dir.to_str().unwrap()),
            Repo::CannotFindLitRepo => "Cannot find lit repository at current path".to_string(),
            Repo::NotDirectory(dir) => format!("`{}` is not a directory", dir.to_str().unwrap()),
            Repo::NotEmpty(dir) => {
                format!("The directory `{}` is not empty", dir.to_str().unwrap())
            }
            Repo::UnsupportedRepositoryFormatVersion(version) => {
                format!("Unsupported repositoryformatversion `{}`", version)
            }
            Repo::MissingConfigFile(config_file_path) => format!(
                "Missing configuration file `{}`",
                config_file_path.to_str().unwrap()
            ),
        }
    }
}
