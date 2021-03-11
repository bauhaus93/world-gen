use std::io;
use std::num;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("io: {source}")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("parse float: {source}")]
    ParseFloat {
        #[from]
        source: num::ParseFloatError,
    },
    #[error("parse int: {source}")]
    ParseInt {
        #[from]
        source: num::ParseIntError,
    },
    #[error("unexpected format: {0}")]
    UnexpectedFormat(String),
    #[error("inconsistent data: {0}")]
    InconsistentData(String),
}
