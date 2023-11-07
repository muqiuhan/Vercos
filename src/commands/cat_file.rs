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
        let _object = object::read(
            &repo,
            Self::object_find(&repo, &self.object, &self.typ, true).as_str(),
        );
    }

    /// The reason for this strange small function is that
    /// lit has a lot of ways to refer to objects: full hash, short hash, tags...
    /// This function is the name resolution function.
    fn object_find(_repo: &Repo, name: &String, _fmt: &String, _follow: bool) -> String {
        name.clone()
    }
}
