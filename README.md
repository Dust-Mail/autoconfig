# Autoconfig

A simple implementation of [Mozilla Thunderbird's autoconfig](https://wiki.mozilla.org/Thunderbird:Autoconfiguration) in Rust.

Useful if a user needs to fill in their mail server configuration, but are not tech savy enough to do so or just for general convenience of not having to manually fill anything in.

Used in Dust-Mail to automatically discover email servers from a users email address.

## Usage

You can request a config by simply calling the `from_addr` function:

```rust
extern crate autoconfig;

#[tokio::main]
async fn main() {
    let config = autoconfig::from_addr("test@gmail.com").await.unwrap();

    println!("{}", config.email_provider().id())

    // Outputs:
    // "googlemail.com"
}
```

You can also achieve the same thing but from just a domain name:

```rust
extern crate autoconfig;

#[tokio::main]
async fn main() {
    let config = autoconfig::from_domain("gmail.com").await.unwrap();

    println!("{}", config.email_provider().id())

    // Outputs:
    // "googlemail.com"
}
```
