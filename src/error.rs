use std::{error, fmt, result};

use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum ErrorKind {
    Surf(surf::Error),
    BuildHttpClient,
    InvalidResponse,
    Timeout,
    BadInput,
    NoRecordsFound,
    Resolve(ResolveError),
    NotFound(Vec<Error>),
    ParseXml(serde_xml_rs::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(kind: ErrorKind, msg: S) -> Self {
        Self {
            kind,
            message: msg.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<surf::Error> for Error {
    fn from(error: surf::Error) -> Self {
        Self::new(ErrorKind::Surf(error), "Failed to create http request")
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(error: serde_xml_rs::Error) -> Self {
        Self::new(ErrorKind::ParseXml(error), "Error parsing XML response")
    }
}

impl From<ResolveError> for Error {
    fn from(error: ResolveError) -> Self {
        Self::new(ErrorKind::Resolve(error), "Error resolving dns")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = result::Result<T, Error>;
