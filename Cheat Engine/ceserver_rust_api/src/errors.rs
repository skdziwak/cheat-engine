use std::num::ParseIntError;

pub struct Error(pub String);

impl Into<String> for Error {
    fn into(self) -> String {
        self.0.clone()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(err.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error(err.to_string())
    }
}