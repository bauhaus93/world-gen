use std::fmt;
use std::error::Error;
use std::io;
use std::num;

#[derive(Debug)]
pub enum FileError {
    IO(io::Error),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    UnexpectedFormat(String)
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> FileError {
        FileError::IO(err)
    }
}

impl From<num::ParseFloatError> for FileError {
    fn from(err: num::ParseFloatError) -> FileError {
        FileError::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for FileError {
    fn from(err: num::ParseIntError) -> FileError {
        FileError::ParseInt(err)
    }
}

impl Error for FileError {

    fn description(&self) -> &str {
        match *self {
            FileError::IO(_) => "io",
            FileError::ParseFloat(_) => "parse float",
            FileError::ParseInt(_) => "parse int",
            FileError::UnexpectedFormat(_) => "unexpected format"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            FileError::IO(ref err) => Some(err),
            FileError::ParseFloat(ref err) => Some(err),
            FileError::ParseInt(ref err) => Some(err),
            FileError::UnexpectedFormat(_) => None
        }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileError::IO(ref err) => write!(f, "{}:{}", self.description(), err),
            FileError::ParseFloat(ref err) => write!(f, "{}:{}", self.description(), err),
            FileError::ParseInt(ref err) => write!(f, "{}:{}", self.description(), err),            
            FileError::UnexpectedFormat(ref unexpected_str) => write!(f, "{}: {}", self.description(), unexpected_str)
        }
    }
}
