use std::{error::Error as MyError, fmt};

pub enum RpError {
    Unexpected
}

impl fmt::Display for RpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::InvalidUrl => write!(f, "InvalidUrl"),
        }
    }  // fmt
}
