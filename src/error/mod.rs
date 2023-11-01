use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Repo(Repo),
}

#[derive(Debug)]
pub enum Repo {
    NotLitRepo(PathBuf),
    NotDirectory(PathBuf),
    NotEmpty(PathBuf),
    MissingConfigFile(PathBuf),
    UnsupportedRepositoryFormatVersion(String),
}

impl Repo {
    pub fn panic(self) -> ! {
        match self {
            Repo::NotLitRepo(dir) => panic!("{} is not a lit repository", dir.to_str().unwrap()),
            Repo::NotDirectory(dir) => panic!("{} is not a directory", dir.to_str().unwrap()),
            Repo::NotEmpty(dir) => panic!("The directory {} is not empty", dir.to_str().unwrap()),
            Repo::UnsupportedRepositoryFormatVersion(version) => {
                panic!("Unsupported repositoryformatversion {}", version)
            }
            Repo::MissingConfigFile(config_file_path) => panic!(
                "Missing configuration file {}",
                config_file_path.to_str().unwrap()
            ),
        }
    }
}

impl Error {
    pub fn panic(self) -> ! {
        match self {
            Error::Repo(err) => err.panic(),
        }
    }
}
