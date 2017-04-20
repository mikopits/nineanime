use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(::hyper::Error),
    Json(::serde_json::Error),
    Parse(::hyper::error::ParseError),
    Read(::std::io::Error),
    Tls(::hyper_native_tls::native_tls::Error),
    ParseError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => fmt::Display::fmt(e, f),
            Error::Json(ref e) => fmt::Display::fmt(e, f),
            Error::Parse(ref e) => fmt::Display::fmt(e, f),
            Error::Read(ref e) => fmt::Display::fmt(e, f),
            Error::Tls(ref e) => fmt::Display::fmt(e, f),
            Error::ParseError => f.pad("Failed to parse html")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Parse(ref e) => e.description(),
            Error::Read(ref e) => e.description(),
            Error::Tls(ref e) => e.description(),
            Error::ParseError => "Failed to parse html"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
            Error::Read(ref e) => Some(e),
            Error::Tls(ref e) => Some(e),
            Error::ParseError => None
        }
    }
}

impl From<::hyper::Error> for Error {
    fn from(err: ::hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<::hyper::error::ParseError> for Error {
    fn from(err: ::hyper::error::ParseError) -> Error {
        Error::Parse(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Read(err)
    }
}

impl From<::hyper_native_tls::native_tls::Error> for Error {
    fn from(err: ::hyper_native_tls::native_tls::Error) -> Error {
        Error::Tls(err)
    }
}

/// A `Result` alias where the `Err` case is `nimeanime::Error`
pub type Result<T> = ::std::result::Result<T, Error>;
