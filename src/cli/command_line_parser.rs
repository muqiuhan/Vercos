/// Copyright (C) 2023 Muqiu Han
use crate::r#const::project_info;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = project_info::NAME, about = project_info::DESCRIPTION)]
pub enum CommandLineParser {
    /// Create an empty lit repository or reinitialize an existing one
    Init {
        /// Force initialization
        #[structopt(short)]
        force: bool,

        /// The repository path, defaults to the current directory (.)
        #[structopt(default_value = ".")]
        path: String,
    },

    /// Provide contents or details of repository objects
    CatFile {
        /// Specify the type (blob | commit | tag | tree)
        #[structopt(default_value = "blob", name = "type", short)]
        typ: String,

        /// The object to display
        #[structopt(name = "object")]
        object: String,
    },
}

impl CommandLineParser {
    pub fn parse() -> Self {
        CommandLineParser::from_args()
    }
}
