# Syntropy PubSub-Rust

`pubsub-rust` is a Rust library for the Syntropy DataMesh project that enables you to subscribe to existing data streams or publish new ones. This library is built on top of the NATS messaging system and provides a convenient way to integrate your Rust applications with the Syntropy DataMesh platform.

## Features

- Subscribe to existing data streams
- Publish new data streams

## Usage
Here is a simple example demonstrating how to subscribe to a data stream and publish:

### The preferred method of authentication is using an access token from the developer portal.

### cargo.toml
```toml
[package]
name = "testrust"
version = "0.1.0"
edition = "2021"

[dependencies]
async-nats = "0.29.0"
tokio = { version = "1.25.0", features = ["full"] }
pubsub-rust = { git = "https://github.com/SyntropyNet/pubsub-rust.git", branch = "main" }
futures = { version = "0.3.26", default-features = false, features = ["std", "async-await"] }
bytes = "1.4.0"
```

### Publish
```rust
extern crate pubsub_rust;

use bytes::Bytes;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "SAAGNJOZTRPYYXG2NJX3ZNGXYUSDYX2BWO447W3SHG6XQ7U66RWHQ3JUXM";
    let nats_server_ip = "nats://127.0.0.1";
    let subject = String::from("example.subject");

    let client = pubsub_rust::connect(nats_server_ip, access_token).await?;

    let now = Instant::now();
    let data = Bytes::from("bar");
    for _ in 0..100 {
        print!("publishing\n");
        io::stdout().flush().unwrap();
        client.publish(subject.clone(), data.clone()).await?;
        sleep(Duration::from_secs(1)).await;  // Wait for 1 second before the next publish
    }
    client.flush().await?;

    println!("published in {:?}", now.elapsed());

    Ok(())
}
```

### Subscribe
```rust
extern crate pubsub_rust;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let access_token = "SAAGNJOZTRPYYXG2NJX3ZNGXYUSDYX2BWO447W3SHG6XQ7U66RWHQ3JUXM";
    let nats_server_ip = "nats://127.0.0.1";
    let subscribe_subject = "example.subject";

    let client = pubsub_rust::connect(nats_server_ip, access_token).await?;

    let now = Instant::now();
    let mut subscriber = client.subscribe(subscribe_subject.into()).await.unwrap();

    println!("Awaiting messages");
    // Import StreamExt here:
    use futures::stream::StreamExt;
    while let Some(message) = subscriber.next().await {
        println!("Received message {:?}", message);
    }

    println!("subscriber received in {:?}", now.elapsed());

    Ok(())
}
```
Those examples demonstrate how to connect to a NATS server, subscribe to a subject, and publish messages to subject using the Syntropy PubSub-Rust library.

## License
This project is licensed under the MIT License.