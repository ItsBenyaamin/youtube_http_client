use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    UrlParsingError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParsingError => write!(f, "Error occure on parsing URL!"),
        }
    }
}