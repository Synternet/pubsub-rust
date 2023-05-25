# Syntropy PubSub-Rust

`syntropy_pubsub` is a Rust library for the Syntropy DataMesh project that enables you to subscribe to existing data streams or publish new ones. This library is built on top of the NATS messaging system and provides a convenient way to integrate your Rust applications with the Syntropy DataMesh platform.

## Features

- Subscribe to existing data streams
- Publish new data streams
- Support for JSON messages
- Customizable connection options

## Installation

To add the library to your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
syntropy_pubsub = "0.1.0"
```

## Usage
Here is a simple example demonstrating how to subscribe to a data stream and publish:

### Publish
```rust
use bytes::Bytes;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use std::io::{self, Write}; // Add this line

mod pubsub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = "SAAGNJOZTRPYYXG2NJX3ZNGXYUSDYX2BWO447W3SHG6XQ7U66RWHQ3JUXM";
    let nats_server_ip = "nats://127.0.0.1";

    let client = pubsub::connect(nats_server_ip, seed).await?;

    let now = Instant::now();
    let subject = String::from("foo");
    let dat = Bytes::from("bar");
    for _ in 0..100 { // Reduced the number of messages for testing
        print!("publishing\n");
        io::stdout().flush().unwrap();
        client.publish(subject.clone(), dat.clone()).await?;
        sleep(Duration::from_secs(1)).await;  // Wait for 1 second before the next publish
    }
    client.flush().await?;

    println!("published in {:?}", now.elapsed());

    Ok(())
}
```

### Subscribe
```rust
use futures::stream::StreamExt;
use std::time::Instant;
mod pubsub;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {

    let seed = "SAAGNJOZTRPYYXG2NJX3ZNGXYUSDYX2BWO447W3SHG6XQ7U66RWHQ3JUXM";
    let nats_server_ip = "nats://127.0.0.1";

    let client = pubsub::connect(nats_server_ip, seed).await?;

    let now = Instant::now();
    let mut subscriber = client.subscribe("foo".into()).await.unwrap();

    println!("Awaiting messages");
    while let Some(message) = subscriber.next().await {
        println!("Received message {message:?}");
    }

    println!("subscriber received in {:?}", now.elapsed());

    Ok(())
}
```
Those examples demonstrate how to connect to a NATS server, subscribe to a subject, and publish messages to subject using the Syntropy PubSub-Rust library.

## License
This project is licensed under the MIT License.