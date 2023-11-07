/// Copyright (C) 2023 Muqiu Han
pub mod object;
pub mod repo;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Repo(repo::Repo),
    Object(object::Object),
}

pub trait Log {
    fn fmt(&self) -> String;

    fn panic(&self) -> ! {
        error!("{}", self.fmt());
        panic!("{}", self.fmt());
    }
}
