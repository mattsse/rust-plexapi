use hyper;
use hyper::StatusCode;
use serde_xml_rs;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};

#[derive(Debug)]
pub enum APIError {
    ReadError,
    HttpError(StatusCode),
    XmlError(serde_xml_rs::Error),
    HyperError(hyper::Error),
    ParseError(String),
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Error! {}. ({:?})", self.description(), self)
    }
}

impl Error for APIError {
    fn description(&self) -> &str {
        match *self {
            APIError::ParseError(_) => "An error occured while parsing.",
            APIError::HttpError(_) => "The API returned a non-success error code",
            APIError::HyperError(_) => "An error occurred while processing the HTTP response",
            APIError::XmlError(_) => {
                "The Xml sent by Plex did not match what the plexapi was expecting"
            }
            _ => "This error should not have occurred. Please file a bug",
        }
    }
}

impl From<hyper::Error> for APIError {
    fn from(err: hyper::Error) -> APIError {
        APIError::HyperError(err)
    }
}

impl From<serde_xml_rs::Error> for APIError {
    fn from(err: serde_xml_rs::Error) -> APIError {
        APIError::XmlError(err)
    }
}