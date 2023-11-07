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

mod blob;
mod reading;
mod writing;

pub trait Object {
    fn to_string(&self) -> String;
    fn deserialize(data: &str) -> Self
    where
        Self: Sized;

    /// It must read the object's contents from data, a byte string, and do
    /// whatever it takes to convert it into a meaningful representation.
    fn serialize(&self) -> &[u8];

    fn fmt(&self) -> &String;
}
