use std::collections::HashMap;

#[tokio::test]
pub async fn from_domain() {
    let mut addresses = HashMap::new();

    addresses.insert("outlook.com", "outlook.com");
    addresses.insert("gmail.com", "googlemail.com");
    addresses.insert("yahoo.com", "yahoo.com");

    for (addr, id) in addresses.iter() {
        let config = super::from_domain(addr).await.unwrap();

        assert_eq!(id, &config.email_provider().id());
    }
}
