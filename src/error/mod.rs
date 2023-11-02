use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Repo(Repo),
}

pub trait Log {
    fn fmt(&self) -> String;

    fn panic(&self) -> ! {
        error!("{}", self.fmt());
        panic!("{}", self.fmt());
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Repo {
    NotLitRepo(PathBuf),
    NotDirectory(PathBuf),
    NotEmpty(PathBuf),
    MissingConfigFile(PathBuf),
    UnsupportedRepositoryFormatVersion(String),
}

impl Log for Repo {
    fn fmt(&self) -> String {
        match self {
            Repo::NotLitRepo(dir) => format!("{} is not a lit repository", dir.to_str().unwrap()),
            Repo::NotDirectory(dir) => format!("{} is not a directory", dir.to_str().unwrap()),
            Repo::NotEmpty(dir) => format!("The directory {} is not empty", dir.to_str().unwrap()),
            Repo::UnsupportedRepositoryFormatVersion(version) => {
                format!("Unsupported repositoryformatversion {}", version)
            }
            Repo::MissingConfigFile(config_file_path) => format!(
                "Missing configuration file {}",
                config_file_path.to_str().unwrap()
            ),
        }
    }
}
