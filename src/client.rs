use regex::Regex;
use reqwest::Url;

use crate::{config::Config, dns::Dns, error::Result, http::Http, parse};

pub struct Client {
    http: Http,
    dns: Dns,
}

impl Client {
    pub fn new() -> Result<Self> {
        let http = Http::new()?;
        let dns = Dns::new()?;
        let client = Self { http, dns };

        Ok(client)
    }

    pub async fn get_config<U: AsRef<str>>(&self, url: U) -> Result<Config> {
        let bytes = self.http.get_xml(url).await?;

        let config = parse::from_bytes(bytes)?;

        Ok(config)
    }

    const TXT_RECORD_REGEX: &str = r"^mailconf=(https?://\S+)$";

    pub async fn get_url_from_txt<N: AsRef<str>>(&self, name: N) -> Result<Vec<String>> {
        let records = self.dns.get_txt(name).await?;

        let re = Regex::new(Self::TXT_RECORD_REGEX).unwrap();

        let mut urls = Vec::new();

        for record in records {
            if let Some(record_str) = std::str::from_utf8(&record).ok() {
                if let Some(captured) = re.captures(record_str) {
                    if let Some(r#match) = captured.get(1) {
                        let url = r#match.as_str();

                        if let Some(url_parsed) = Url::parse(url).ok() {
                            if url_parsed.scheme() == "https" {
                                urls.push(url.to_string())
                            }
                        }
                    }
                }
            }
        }

        Ok(urls)
    }
}
