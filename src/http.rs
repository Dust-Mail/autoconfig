use std::time::Duration;

use crate::error::{Error, ErrorKind, Result};

use bytes::Bytes;
use reqwest::{redirect, Client as HttpClient};

pub struct Http {
    client: HttpClient,
}

impl Http {
    const TIMEOUT: Duration = Duration::from_secs(10);
    const MAX_REDIRECTS: usize = 10;
    /// The accepted content types for an xml response
    const XML_CONTENT_TYPE: (&str, &str) = ("application/xml", "text/xml");

    pub fn new() -> Result<Self> {
        let redirect_policy = redirect::Policy::limited(Self::MAX_REDIRECTS);

        let client = HttpClient::builder()
            .timeout(Self::TIMEOUT)
            .redirect(redirect_policy)
            .build()?;

        Ok(Self { client })
    }

    /// Fetches a given url and returns the XML response (if there is one)
    pub async fn get_xml<S: AsRef<str>>(&self, uri: S) -> Result<Bytes> {
        let response = self.client.get(uri.as_ref()).send().await?;

        // Get the Content-Type header, error if it doesn't exist
        let content_type = match response.headers().get("content-type") {
            Some(header) => header.to_str().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidResponse,
                    "Content-Type header does not contain valid characters",
                )
            })?,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidResponse,
                    "Server did not include a content-type header in response",
                ))
            }
        };

        // Ensure the content type is XML
        if !(content_type.starts_with(Self::XML_CONTENT_TYPE.0)
            || content_type.starts_with(Self::XML_CONTENT_TYPE.1))
        {
            return Err(Error::new(
                ErrorKind::InvalidResponse,
                "Server did not respond with xml content",
            ));
        }

        let is_success = response.status().is_success();

        // Get the http message body
        let bytes = response.bytes().await?;

        // If we got an error response we return an error
        if !is_success {
            return Err(Error::new(
                ErrorKind::InvalidResponse,
                format!(
                    "Http request failed: {}",
                    String::from_utf8(bytes.into()).unwrap()
                ),
            ));
        } else {
            return Ok(bytes);
        };
    }
}
