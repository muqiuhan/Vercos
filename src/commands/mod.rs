/// Copyright (C) 2023 Muqiu Han

use crate::cli::CommandLineParser;

pub mod init;

pub fn command(args: &CommandLineParser) {
    match args {
        CommandLineParser::Init { path, force } => {
            info!("create lit repository on {}...", path);
            init::Init::create(path.clone(), *force);
            info!("create ok!");
        }
    }
}
