use crate::r#const::project_info;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = project_info::NAME, about = project_info::DESCRIPTION)]
pub enum CommandLineParser {
    /// Create an empty Git repository or reinitialize an existing one
    Init {
        /// Force initialization
        #[structopt(short)]
        force: bool,

        /// The repository path, defaults to the current directory (.)
        #[structopt(default_value = ".")]
        path: String,
    },
    // Commit {
    //     #[structopt(short)]
    //     message: Option<String>,
    //     #[structopt(short)]
    //     all: bool,
    // },
}

impl CommandLineParser {
    pub fn parse() -> Self {
        CommandLineParser::from_args()
    }
}
