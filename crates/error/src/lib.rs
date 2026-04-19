use serde::ser::SerializeStruct;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotFoundError(String),
    InvalidInputError(String),
    IOError(String),
    FSError(String),
    ParseError(String),
    SerializeError(String),
    DeserializeError(String),
    DatabaseError(String),
    ChannelError(String),
}
impl std::error::Error for Error {}

impl Error {
    pub fn kind(&self) -> &str {
        match self {
            Error::NotFoundError(_) => "NotFoundError",
            Error::InvalidInputError(_) => "InvalidInputError",
            Error::IOError(_) => "IOError",
            Error::FSError(_) => "FSError",
            Error::ParseError(_) => "ParseError",
            Error::SerializeError(_) => "SerializeError",
            Error::DeserializeError(_) => "DeserializeError",
            Error::DatabaseError(_) => "DatabaseError",
            Error::ChannelError(_) => "ChannelError",
        }
    }
    pub fn message(&self) -> &str {
        match self {
            Error::NotFoundError(err_str)
            | Error::InvalidInputError(err_str)
            | Error::IOError(err_str)
            | Error::FSError(err_str)
            | Error::ParseError(err_str)
            | Error::SerializeError(err_str)
            | Error::DeserializeError(err_str)
            | Error::DatabaseError(err_str)
            | Error::ChannelError(err_str) => err_str.as_str(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::InvalidInputError(err_str) => write!(f, "InvalidInputError: {}", err_str),
            Error::IOError(err_str) => write!(f, "IOError: {}", err_str),
            Error::FSError(err_str) => write!(f, "FSError: {}", err_str),
            Error::ParseError(err_str) => write!(f, "ParseError: {}", err_str),
            Error::SerializeError(err_str) => write!(f, "SerializeError: {}", err_str),
            Error::DeserializeError(err_str) => write!(f, "DeserializeError: {}", err_str),
            Error::DatabaseError(err_str) => write!(f, "DatabaseError: {}", err_str),
            Error::ChannelError(err_str) => write!(f, "ChannelError: {}", err_str),
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("message", self.message())?;
        s.end()
    }
}

#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::DatabaseError(value.to_string())
    }
}
#[cfg(feature = "tokio")]
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelError(value.to_string())
    }
}
#[cfg(feature = "axum")]
impl From<axum::http::header::InvalidHeaderValue> for Error {
    fn from(value: axum::http::header::InvalidHeaderValue) -> Self {
        Error::InvalidInputError(value.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::DeserializeError(value.to_string())
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseError(value.to_string())
    }
}
