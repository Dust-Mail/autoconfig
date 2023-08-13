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

use http::Client;
use types::{config::Config, Error, ErrorKind, Result};
use utils::validate_email;

mod http;
mod parse;
pub mod types;
mod utils;

const AT_SYMBOL: char = '@';

/// Given an email providers domain, try to connect to autoconfig servers for that provider and return the config.
pub async fn from_domain<D: AsRef<str>>(domain: D) -> Result<Config> {
    let client = Client::new()?;

    let urls = vec![
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

    let config_unparsed: Option<String> = client.request_urls(urls).await;

    match config_unparsed {
        Some(config_unparsed) => {
            let config = parse::from_str(&config_unparsed)?;

            Ok(config)
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Could not find a valid config",
        )),
    }
}

/// Given an email address, try to connect to the email providers autoconfig servers and return the config that was found, if one was found.
pub async fn from_addr(email_address: &str) -> Result<Config> {
    if !validate_email(email_address) {
        return Err(types::Error::new(
            types::ErrorKind::BadInput,
            "Given email address is invalid",
        ));
    };

    let mut split = email_address.split(AT_SYMBOL);

    // Skip the prefix
    split.next();

    let domain = match split.next() {
        Some(domain) => domain,
        None => {
            return Err(types::Error::new(
                types::ErrorKind::BadInput,
                "An email address must specify a domain after the '@' symbol",
            ))
        }
    };

    from_domain(domain).await
}

#[cfg(test)]
mod test;
