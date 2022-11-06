use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    UrlParsingError,
    IoError(String),
    Utf8ParsingError,
    HeaderParsingError
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParsingError => write!(f, "An error occurred on parsing the URL!"),
            Error::IoError(e) => write!(f, "An error occurred on IO operation: {}", e),
            Error::Utf8ParsingError => write!(f, "An error occurred on parsing response!"),
            Error::HeaderParsingError => write!(f, "An error occurred on parsing headers of the response!"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::Utf8ParsingError
    }
}