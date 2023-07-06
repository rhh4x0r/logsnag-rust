# A Logsnag API Wrapper for Rust

[![Crate](https://img.shields.io/crates/v/logsnag.svg)](https://crates.io/crates/logsnag)
[![API](https://docs.rs/logsnag/badge.svg)](https://docs.rs/logsnag)

`logsnag` is a simple and efficient Rust library for interacting with the [Logsnag](https://docs.logsnag.com/endpoints/log) API. It supports asynchronous requests and allows easy publication of logs and insights.

## Features

- Publish logs to channels with specified event, optional description, icon, and notify flag
- Publish insights with a specified title, event, value, and an optional icon
- [TODO] Support tags
- [TODO] Support validation for inputs on the strings pre-runtime
## Getting Started

First, add `logsnag` to your `Cargo.toml`:

```toml
[dependencies]
logsnag = "0.3.1"
```
Then, import it in your file(s).

```rust
use logsnag::Logsnag;
use logsnag::models::InsightValue; //Only required for Insights
```

## Usage

Here is a basic example of how to use the `Logsnag` client:

```rust
use logsnag::Logsnag;
use logsnag::models::InsightValue;

async fn main() {
    let logsnag = Logsnag::new(
        env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key found"), 
        env::var("LOGSNAG_PROJECT").expect("No Logsnag Project found")
    );

    //Use the following for Raw Strings
    //let client = Logsnag::new("my-api-token".to_string(), "my-project".to_string());

    let publish_response = logsnag.publish(
        "channel".to_string(),
        "event".to_string(),
        Some("description".to_string()),
        Some("❤️".to_string()),
        Some(true)
    ).await.expect("Failed to publish log");

    let insight_response = logsnag.insight(
        "title".to_string(), 
        "event".to_string(), 
        InsightValue::Str("online"), //InsightValue::Int(69), InsightValue::Bool(false)
        Some("❤️".to_string())
    ).await.expect("Failed to publish insight");
}
```

See the [API Documentation](https://docs.rs/logsnag) for more details.

## Contributing

Please feel free to submit issues, fork the repository and send pull requests!

Any questions, you can find me (rhh4x0r) on the [Official Logsnag Discord Server](https://discord.gg/udRNTt7xCJ) or submit an issue.
