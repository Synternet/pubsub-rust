Welcome to the documentation for the Rust SDK for the Data Availability Layer! This SDK allows seamless integration with our Data Availability Layer solution, enabling you to leverage real-time data streams in your Rust applications. With the Rust SDK, you can unlock the power of the Data Availability Layer and harness real-time insights for your data-driven projects.

[pubsub-rust](https://github.com/SyntropyNet/pubsub-rust) is a Rust library for the Syntropy DataMesh project that enables you to subscribe to existing data streams or publish new ones. This library is built on top of the NATS messaging system and provides a convenient way to integrate your Rust applications with the Syntropy DataMesh platform.

# Features

The Rust SDK for Data Availability Layer offers the following features:

- **Subscribe to Existing Data Streams**: Easily subscribe to pre-existing data streams within the Syntropy Data Availability Layer. Stay updated with real-time data insights and leverage them in your Rust applications.
- **Publish New Data Streams**: Create and publish your own data streams directly from your Rust applications. Share data with other participants in the Data Availability Layer, enabling collaboration and innovation.

# Installation

To use the Rust SDK for Data Availability Layer in your project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
syntropynet_pubsub = "0.1.0"
```

# Getting Started

Before you begin using the Rust SDK, make sure you have the necessary credentials and access tokens from the SyntropyNet platform. These credentials will allow you to connect to the Data Availability Layer and subscribe to or publish data streams.

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
client.publish("stream-name", b"Hello, Data Availability Layer!").await?;
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

## Contribution Guidelines

To contribute to this project, please follow the guidelines outlined in the [Contribution.md](CONTRIBUTING.md) file. It covers important information about how to submit bug reports, suggest new features, and submit pull requests.

## Code of Conduct
This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md) to ensure a welcoming and inclusive environment for all contributors. Please review the guidelines and make sure to follow them in all interactions within the project.

## Commit Message Format
When making changes to the codebase, it's important to follow a consistent commit message format. Please refer to the [Commit Message Format](commit-template.md) for details on how to structure your commit messages.

## Pull Request Template
To streamline the pull request process, we have provided a [Pull Request Template](pull-request-template.md) that includes the necessary sections for describing your changes, related issues, proposed changes, and any additional information. Make sure to fill out the template when submitting a pull request.

We appreciate your contributions and thank you for your support in making this project better!

## Support

If you encounter any difficulties or have questions regarding the Rust SDK for Data Availability Layer, please reach out to our support team at support@syntropynet.com. We are here to assist you and ensure a smooth experience with our SDK.

We hope this documentation provides you with a comprehensive understanding of the Rust SDK for the Data Availability Layer. Enjoy leveraging real-time data streams and unlocking the power of the Data Availability Layer in your Rust applications!