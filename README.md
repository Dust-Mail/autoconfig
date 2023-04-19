# Autoconfig

![Crates.io (latest)](https://img.shields.io/crates/dv/autoconfig)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/dust-mail/autoconfig/test.yml)

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

## Example

Below is an example shown of how the config struct might look like:

```rust
Config {
        version: "1.1",
        email_provider: EmailProvider {
            id: "googlemail.com",
            properties: [
                Domain("gmail.com"),
                Domain("googlemail.com"),
                Domain("google.com"),
                DisplayName("Google Mail"),
                DisplayShortName("GMail"),
                IncomingServer(Server {
                    r#type: Imap,
                    properties: [
                        Hostname("imap.gmail.com"),
                        Port(993),
                        SocketType(Tls),
                        Username("%EMAILADDRESS%"),
                        Authentication(OAuth2),
                        Authentication(PasswordCleartext),
                    ],
                }),
                IncomingServer(Server {
                    r#type: Pop3,
                    properties: [
                        Hostname("pop.gmail.com"),
                        Port(995),
                        SocketType(Tls),
                        Username("%EMAILADDRESS%"),
                        Authentication(OAuth2),
                        Authentication(PasswordCleartext),
                        Pop3(Pop3Config {
                            leave_messages_on_server: true,
                            download_on_biff: None,
                            days_to_leave_messages_on_server: None,
                            check_interval: None,
                        }),
                    ],
                }),
                OutgoingServer(Server {
                    r#type: Smtp,
                    properties: [
                        Hostname("smtp.gmail.com"),
                        Port(465),
                        SocketType(Tls),
                        Username("%EMAILADDRESS%"),
                        Authentication(OAuth2),
                        Authentication(PasswordCleartext),
                    ],
                }),
                Documentation(Documentation {
                    url: "http://mail.google.com/support/bin/answer.py?answer=13273",
                    properties: [
                        DocumentationDescription {
                            lang: None,
                            description: "How to enable IMAP/POP3 in GMail",
                        },
                    ],
                }),
                Documentation(Documentation {
                    url: "http://mail.google.com/support/bin/topic.py?topic=12806",
                    properties: [
                        DocumentationDescription {
                            lang: None,
                            description: "How to configure email clients for IMAP",
                        },
                    ],
                }),
                Documentation(Documentation {
                    url: "http://mail.google.com/support/bin/topic.py?topic=12805",
                    properties: [
                        DocumentationDescription {
                            lang: None,
                            description: "How to configure email clients for POP3",
                        },
                    ],
                }),
                Documentation(Documentation {
                    url: "http://mail.google.com/support/bin/answer.py?answer=86399",
                    properties: [
                        DocumentationDescription {
                            lang: None,
                            description: "How to configure TB 2.0 for POP3",
                        },
                    ],
                }),
            ],
        },
        oauth2: Some(OAuth2Config {
            issuer: "accounts.google.com",
            scope: "https://mail.google.com/ https://www.googleapis.com/auth/contacts https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/carddav",
            auth_url: "https://accounts.google.com/o/oauth2/auth",
            token_url: "https://www.googleapis.com/oauth2/v3/token",
        }),
    };
```
