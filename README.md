Welcome to the documentation for the Rust SDK for the Data Mesh! This SDK allows seamless integration with our data mesh solution, enabling you to leverage real-time data streams in your Rust applications. With the Rust SDK, you can unlock the power of the Data Mesh and harness real-time insights for your data-driven projects.

[pubsub-rust](https://github.com/SyntropyNet/pubsub-rust) is a Rust library for the Syntropy DataMesh project that enables you to subscribe to existing data streams or publish new ones. This library is built on top of the NATS messaging system and provides a convenient way to integrate your Rust applications with the Syntropy DataMesh platform.

# Features

The Rust SDK for Data Mesh offers the following features:

- **Subscribe to Existing Data Streams**: Easily subscribe to pre-existing data streams within the Syntropy Data Mesh. Stay updated with real-time data insights and leverage them in your Rust applications.
- **Publish New Data Streams**: Create and publish your own data streams directly from your Rust applications. Share data with other participants in the Data Mesh, enabling collaboration and innovation.

# Installation

To use the Rust SDK for Data Mesh in your project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
syntropynet_pubsub = "0.1.0"
```

# Getting Started

Before you begin using the Rust SDK, make sure you have the necessary credentials and access tokens from the SyntropyNet platform. These credentials will allow you to connect to the Data Mesh and subscribe to or publish data streams.

## Usage

1. Import the SDK:

```rust
use syntropynet_pubsub::DataMeshClient;
```

2. Initialize the client:

```rust
let client = DataMeshClient::new("your-access-token", "your-private-key");
```

3. Subscribe to a Data Stream:

```rust
let stream = client.subscribe("stream-name").await?;
```

4. Receive Data Stream Events:

```rust
while let Some(event) = stream.next().await {
    // Handle the data stream event
    println!("Received event: {:?}", event);
}
```

5. Publish Data to a Stream:

```rust
client.publish("stream-name", b"Hello, Data Mesh!").await?;
```

## Examples

For detailed usage examples, please refer to the [examples directory](https://github.com/SyntropyNet/pubsub-rust/examples) in the repository. These examples cover various scenarios and demonstrate how to utilize the SDK's features effectively.

The preferred method of authentication is using an access token from the developer portal.

### `cargo.toml`

```Text TOML
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

```Text Rust
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

```Text Rust
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

## Contributing

We welcome contributions from the community! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/SyntropyNet/pubsub-rust). We appreciate your feedback and collaboration in making this SDK even better.

## Support

If you encounter any difficulties or have questions regarding the Rust SDK for Data Mesh, please reach out to our support team at support@syntropynet.com. We are here to assist you and ensure a smooth experience with our SDK.

We hope this documentation provides you with a comprehensive understanding of the Rust SDK for the Data Mesh. Enjoy leveraging real-time data streams and unlocking the power of the Data Mesh in your Rust applications!