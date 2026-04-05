use std::fmt;
use std::io;

#[derive(Debug)]
pub enum XcatError {
    Io(io::Error, String),
    Config(String),
    NoInput,
    Mmap(io::Error, String),
}

impl fmt::Display for XcatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XcatError::Io(err, path) => write!(f, "xcat: {path}: {err}"),
            XcatError::Config(msg) => write!(f, "xcat: config error: {msg}"),
            XcatError::NoInput => write!(f, "xcat: no input files"),
            XcatError::Mmap(err, path) => write!(f, "xcat: mmap error for {path}: {err}"),
        }
    }
}

impl From<io::Error> for XcatError {
    fn from(err: io::Error) -> Self {
        XcatError::Io(err, String::from("unknown"))
    }
}

impl std::error::Error for XcatError {}

pub type XcatResult<T> = Result<T, XcatError>;
