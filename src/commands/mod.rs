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

use crate::cli::CommandLineParser;

pub mod cat_file;
pub mod init;
mod hash_object;

pub fn command(args: CommandLineParser) {
    match args {
        CommandLineParser::Init { force, path } => {
            CommandLineParser::Init { force, path }.init();
        }

        CommandLineParser::CatFile { typ, object } => {
            CommandLineParser::CatFile { typ, object }.cat()
        }

        CommandLineParser::HashObject { typ, write, path } => {
            CommandLineParser::HashObject { typ, write, path }.hash();
        }
    }
}
