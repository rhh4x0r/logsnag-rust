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
logsnag = "0.4.0"
```
Then, import it in your file(s).

```rust
use logsnag::Logsnag;
```

## Usage

Here is a basic example of how to use the `Logsnag` client. It uses a builder to add on to the initial event/insight for the optional parameters. Note that for tags, you can add one at a time with `with_tag()`:

```rust
use logsnag::Logsnag;

async fn main() {

    let logsnag_key = env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in environment variables.");
    let logsnag_project = env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in environment variables.");

    let logsnag = Logsnag::new(
        &logsnag_key, //or pass "your-api-key-here"
        &logsnag_project //or pass "your-logsnag-project-here"
    );

    let publish_result = logsnag.event("channel","event")
        .with_notify(true)
        .with_description("description")
        .with_icon("❤️")
        .with_tag("firsttag", "value")
        .with_tag("secondtag", "secondvalue")
        .publish()
        .await;

    let insight_result = logsnag.insight("Status", "online")
        .with_icon("💀")
        .publish()
        .await;
}
```

See the [API Documentation](https://docs.rs/logsnag) for more details.

## Contributing

Please feel free to submit issues, fork the repository and send pull requests!

Any questions, you can find me (rhh4x0r) on the [Official Logsnag Discord Server](https://discord.gg/udRNTt7xCJ) or submit an issue.
