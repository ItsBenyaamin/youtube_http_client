use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    UrlParsingError,
    IoError
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParsingError => write!(f, "An error occurred on parsing the URL!"),
            Error::IoError => write!(f, "An error occurred on IO operation!"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IoError
    }
}