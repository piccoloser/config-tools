use std::fmt;

#[derive(Debug)]
pub enum Error {
    AlreadyExists,
    NotFound,
    ConfigLoad(ini::Error),
    ConfigCreation(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::AlreadyExists => write!(f, "The key already exists"),
            Error::NotFound => write!(f, "The key was not found"),
            Error::ConfigLoad(e) => write!(f, "Failed to load config file: {e:?}"),
            Error::ConfigCreation(e) => write!(f, "Failed to create config file: {e:?}"),
        }
    }
}
