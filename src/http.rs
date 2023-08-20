use std::time::Duration;

use crate::error::{Error, ErrorKind, Result};

use bytes::Bytes;
use surf::{Client as HttpClient, Config};

pub struct Http {
    client: HttpClient,
}

impl Http {
    const TIMEOUT: Duration = Duration::from_secs(10);

    pub fn new() -> Result<Self> {
        let client: HttpClient = Config::new()
            .set_timeout(Some(Self::TIMEOUT))
            .try_into()
            .map_err(|err| {
                Error::new(
                    ErrorKind::BuildHttpClient,
                    format!("Failed to create http client: {}", err),
                )
            })?;

        Ok(Self { client })
    }

    /// Fetches a given url and returns the XML response (if there is one)
    pub async fn get<S: AsRef<str>>(&self, uri: S) -> Result<Bytes> {
        let mut response = self.client.get(uri.as_ref()).send().await?;

        let is_success = response.status().is_success();

        // Get the http message body
        let bytes = response.body_bytes().await?;

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
            return Ok(bytes.into());
        };
    }
}
