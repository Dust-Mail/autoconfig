use std::collections::HashMap;

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
pub async fn from_domain() {
    let mut addresses = HashMap::new();

    addresses.insert("outlook.com", "outlook.com");
    addresses.insert("gmail.com", "googlemail.com");
    addresses.insert("yahoo.com", "yahoo.com");
    addresses.insert("guusvanmeerveld.dev", "guusvanmeerveld.dev");
    addresses.insert("live.nl", "hotmail.com");

    for (addr, id) in addresses.iter() {
        let config = super::from_domain(addr).await.unwrap();

        assert_eq!(id, &config.email_provider().id());
    }
}
