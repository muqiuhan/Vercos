use crate::cli::CommandLineParser;

pub mod init;

pub fn command(args: &CommandLineParser) {
    match args {
        CommandLineParser::Init { path, force: _ } => {
            info!("create lit repository on {}...", path);
            init::Init::create(path.clone());
            info!("create ok!");
        }
    }
}
