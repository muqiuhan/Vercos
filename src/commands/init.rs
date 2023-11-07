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
use crate::repo::Repo;
use std::fs;

pub struct Init {
    /// Force initialization
    pub force: bool,

    /// The repository path, defaults to the current directory (.)
    pub path: String,
}

impl Init {
    pub fn create(path: String, force: bool) -> Repo {
        let repo = Repo::new(&path, !force);

        info!(
            "create repository worktree on {}",
            &repo.worktree.to_str().unwrap()
        );
        Self::create_worktree(&repo, force);

        info!("create repository directories...");
        Self::create_dirs(&repo);

        info!("create repository files...");
        Self::create_file(&repo);

        info!("create repository configuration file...");
        Self::create_config(&repo);

        repo
    }

    // Make sure the path either doesn't exist or is an empty dir.
    fn create_worktree(repo: &Repo, force: bool) {
        let worktree = &repo.worktree;
        let lit_dir = &repo.lit_dir;

        if repo.worktree.exists() {
            if !(worktree.is_dir()) {
                error::repo::Repo::NotDirectory(worktree.clone()).panic()
            }

            if (lit_dir.exists()) && (lit_dir.read_dir().unwrap().next().is_some()) && (!force) {
                error::repo::Repo::NotEmpty(worktree.clone()).panic()
            }
        } else {
            fs::create_dir_all(worktree).unwrap();
        }
    }

    fn create_dirs(repo: &Repo) {
        Repo::repo_dir(&repo.lit_dir, &["branchs"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["objects"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["refs", "tags"], true).unwrap();
        Repo::repo_dir(&repo.lit_dir, &["refs", "heads"], true).unwrap();
    }

    fn create_file(repo: &Repo) {
        fs::write(
            Repo::repo_file(&repo.lit_dir, &["description"], false).unwrap(),
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .unwrap();

        fs::write(
            Repo::repo_file(&repo.lit_dir, &["HEAD"], false).unwrap(),
            "ref: refs/heads/master\n",
        )
        .unwrap();
    }

    fn create_config(repo: &Repo) {
        let mut conf = ini::Ini::new();
        conf.with_section(Some("core"))
            // The version of the lit_dir format.
            // 0 means the initial format,
            // 1 the same with extensions.
            // If > 1, lit will panic; lit will only accept 0.
            .set("repositoryformatversion", "0")
            // Disable tracking of file mode (permissions) changes in the work tree.
            .set("filemode", "false")
            // Dedicates that this repository has a worktree.
            .set("bare", "false");

        conf.write_to_file(Repo::repo_file(&repo.lit_dir, &["config"], false).unwrap())
            .unwrap()
    }
}
