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

use super::Object;

/// Blobs are user data: the content of every file you put in lit (main.c, logo.png, README.md) is stored as a blob.
pub struct Blob {
    fmt: String,
    data: String,
}

impl Blob {
    pub fn new(data: String) -> Self {
        Blob {
            fmt: "blob".to_string(),
            data,
        }
    }
}

impl Object for Blob {
    fn to_string(&self) -> String {
        self.data.clone()
    }

    fn deserialize(_object: &str) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn serialize(&self) -> &[u8] {
        self.data.as_bytes()
    }

    fn fmt(&self) -> &String {
        &self.fmt
    }
}
