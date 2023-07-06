# A Logsnag API Wrapper for Rust

[![Crate](https://img.shields.io/crates/v/logsnag.svg)](https://crates.io/crates/logsnag)
[![API](https://docs.rs/logsnag/badge.svg)](https://docs.rs/logsnag)

`logsnag` is a simple and efficient Rust library for interacting with the [Logsnag](https://docs.logsnag.com/endpoints/log) API. It supports asynchronous requests and allows easy publication of logs and insights.

## Features

- Publish logs to channels with specified event, optional description, icon, and notify flag
- Publish insights with a specified title, event, value, and an optional icon

## Getting Started

First, add `logsnag` to your `Cargo.toml`:

```toml
[dependencies]
logsnag = "0.2.0"

Then, import it in your file(s):

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
    let client = Logsnag::new("my-api-token", "my-project");

    let response = client.publish(
        "channel",
        "event",
        Some("description"),
        Some("icon"),
        Some(true)
    ).await.expect("Failed to publish log");

    let response = client.insight(
        "my-title", 
        "my-event", 
        InsightValue::new("online"), //or InsightValue::new(10) for numbers 
        Some("❤️")
    ).await.expect("Failed to publish insight");
}
```

See the [API Documentation](https://docs.rs/logsnag) for more details.

## Contributing

Please feel free to submit issues, fork the repository and send pull requests!

Any questions, you can find me (rhh4x0r) on the [Official Logsnag Discord Server](https://discord.gg/udRNTt7xCJ) or submit an issue.
