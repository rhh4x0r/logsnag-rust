# A Logsnag API Wrapper for Rust

[![Crate](https://img.shields.io/crates/v/logsnag.svg)](https://crates.io/crates/logsnag)
[![API](https://docs.rs/logsnag/badge.svg)](https://docs.rs/logsnag)

`logsnag` is a simple and efficient Rust library for interacting with the [Logsnag](https://docs.logsnag.com/endpoints/log) API. It supports asynchronous requests and allows easy publication of logs and insights.

Note: this crate is currently being actively developed. It may change a lot until v1.0. Keep this in mind if you're using it for production apps.

## Features

- Publish logs to channels with specified event, optional description, icon, and notify flag
- Publish insights with a specified title, event, value, and an optional icon
- Support tags
- Support validation for inputs on the strings (some)
## Getting Started

First, add `logsnag` to your `Cargo.toml`:

```toml
[dependencies]
logsnag = "0.6.1"
```
Then, import it in your file(s).

```rust
use logsnag::Logsnag;
```

## Usage

Here is a basic example of how to use the `Logsnag` client. It uses a builder to add on to the initial event/insight for the optional parameters. Anything that has `with_` is an optional function. Note that for tags, you must add one at a time with `with_tag()`:

```rust
use logsnag::Logsnag;

async fn main() {

    let logsnag = Logsnag::new(
        env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in Environment.")
        env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in Environment.")
    );

    let publish_result = logsnag.event("channel","event")
        .with_notify(true)
        .with_description("description")
        .with_icon("❤️")
        .with_tag("tag", "value")
        .with_tag("tag2", "value2")
        .publish()
        .await;

    let insight_result = logsnag.insight("title", "value") //value can be also an Int or a bool
        .with_icon("🟢")
        .publish()
        .await;
}
```


### Auto Validation

Tags have a specific format that they need to be in to pass correctly to publish an event. They need to be lowercase, have no special characters, no spaces, and no uppercase characters. Only dashes are allowed.

`with_tag()` automatically will parse your text and strip the unwanted characters out to prevent runtime errors. There's not a currently known way to check validation on compile time within Rust, but if that changes we will implement it. If you know of a way to do this within functions let us know. 

## Docs

See the [API Documentation](https://docs.rs/logsnag) for more details around the methods we use in this library.

## Contributing

Please feel free to submit issues, fork the repository and send pull requests!

Any questions, you can find me (rhh4x0r) on the [Official Logsnag Discord Server](https://discord.gg/udRNTt7xCJ) or submit an issue.
