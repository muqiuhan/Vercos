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
        #[structopt]
        object: String,
    },

    /// Compute object ID and optionally create an object from a file
    HashObject {
        /// write the object into the object database
        #[structopt(short)]
        write: bool,

        /// Specify the type (blob | commit | tag | tree)
        #[structopt(default_value = "blob", name = "type", short)]
        typ: String,

        #[structopt]
        path: String,
    }
}

impl CommandLineParser {
    pub fn parse() -> Self {
        CommandLineParser::from_args()
    }
}
