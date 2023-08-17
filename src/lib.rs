//!
//! A simple implementation of Mozilla Thunderbird's autoconfig (https://wiki.mozilla.org/Thunderbird:Autoconfiguration) in Rust.
//!
//! Useful if a user needs to fill in their mail server configuration, but are not tech savy enough to do so or just for general convenience of not having to manually fill anything in.
//!
//! # Usage
//!
//! You can request a config by simply calling the `from_addr` function:
//!
//! ```rust,ignore
//!
//! extern crate autoconfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = autoconfig::from_addr("test@gmail.com").await.unwrap();
//!
//!     println!("{}", config.email_provider().id())
//!     
//!     // Outputs:
//!     // "googlemail.com"
//! }
//!
//! ```
//!
//! You can also achieve the same thing but from just a domain name:
//!
//! ```rust,ignore
//!
//! extern crate autoconfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = autoconfig::from_domain("gmail.com").await.unwrap();
//!
//!     println!("{}", config.email_provider().id())
//!     
//!     // Outputs:
//!     // "googlemail.com"
//! }
//!
//! ```
//!

use client::Client;
use futures::{future::select_ok, FutureExt};
use utils::validate_email;

mod client;
pub mod config;
mod dns;
pub mod error;
mod http;
mod parse;
mod utils;

const AT_SYMBOL: char = '@';

use config::Config;
use error::{Error, ErrorKind, Result};

/// Given an email providers domain, try to connect to autoconfig servers for that provider and return the config.
pub async fn from_domain<D: AsRef<str>>(domain: D) -> Result<Config> {
    let mut errors: Vec<_> = Vec::new();

    let client = Client::new()?;

    let mut futures = Vec::new();

    let mut urls = vec![
        // Try connect to connect with the users mail server directly
        format!("http://autoconfig.{}/mail/config-v1.1.xml", domain.as_ref()),
        // The fallback url
        format!(
            "http://{}/.well-known/autoconfig/mail/config-v1.1.xml",
            domain.as_ref()
        ),
        // If the previous two methods did not work then the email server provider has not setup Thunderbird autoconfig, so we ask Mozilla for their config.
        format!(
            "https://autoconfig.thunderbird.net/v1.1/{}",
            domain.as_ref()
        ),
    ];

    match client.get_url_from_txt(domain.as_ref()).await {
        Ok(txt_urls) => {
            for url in txt_urls {
                urls.push(url)
            }
        }
        Err(error) => errors.push(error),
    };

    urls.sort();
    urls.dedup();

    for url in urls {
        let future = client.get_config(url);

        futures.push(future.boxed());
    }

    let result = select_ok(futures).await;

    match result {
        Ok((config, _remaining)) => return Ok(config),
        Err(error) => errors.push(error),
    }

    Err(Error::new(
        ErrorKind::NotFound(errors),
        "Could not find a valid config",
    ))
}

/// Given an email address, try to connect to the email providers autoconfig servers and return the config that was found, if one was found.
pub async fn from_addr(email_address: &str) -> Result<Config> {
    if !validate_email(email_address) {
        return Err(Error::new(
            ErrorKind::BadInput,
            "Given email address is invalid",
        ));
    };

    let mut split = email_address.split(AT_SYMBOL);

    // Skip the prefix
    split.next();

    let domain = match split.next() {
        Some(domain) => domain,
        None => {
            return Err(Error::new(
                ErrorKind::BadInput,
                "An email address must specify a domain after the '@' symbol",
            ))
        }
    };

    from_domain(domain).await
}

#[cfg(test)]
mod test;
