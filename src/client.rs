use crate::{config::Config, error::Result, http::Http, parse};

pub struct Client {
    http: Http,
}

impl Client {
    pub fn new() -> Result<Self> {
        let http = Http::new()?;
        let client = Self { http };

        Ok(client)
    }

    pub async fn get_config<U: AsRef<str>>(&self, url: U) -> Result<Config> {
        let bytes = self.http.get_xml(url).await?;

        let config = parse::from_bytes(bytes)?;

        Ok(config)
    }
}
