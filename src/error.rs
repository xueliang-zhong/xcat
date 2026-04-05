use std::fmt;
use std::io;

#[derive(Debug)]
pub enum XcatError {
    Io(io::Error, String),
    Config(String),
    Mmap(io::Error, String),
}

impl fmt::Display for XcatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err, path) => write!(f, "xcat: {}: {}", path, err),
            Self::Config(msg) => write!(f, "xcat: config error: {}", msg),
            Self::Mmap(err, path) => write!(f, "xcat: mmap error for {}: {}", path, err),
        }
    }
}

impl std::error::Error for XcatError {}

pub type XcatResult<T> = Result<T, XcatError>;
