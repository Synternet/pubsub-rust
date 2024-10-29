Welcome to the documentation for the Rust SDK for the Data Layer! This SDK allows seamless integration with our Data Layer solution, enabling you to leverage real-time data streams in your Rust applications. With the Rust SDK, you can unlock the power of the Data Layer and harness real-time insights for your data-driven projects.

[pubsub-rust](https://github.com/Synternet/pubsub-rust) is a Rust library for the Synternet Data Layer project that enables you to subscribe to existing data streams or publish new ones. This library is built on top of the NATS messaging system and provides a convenient way to integrate your Rust applications with the Synternet Data Layer platform.

# Features

The Rust SDK for Data Layer offers the following features:

- **Subscribe to Existing Data Streams**: Easily subscribe to pre-existing data streams within the Synternet Data Layer. Stay updated with real-time data insights and leverage them in your Rust applications.
- **Publish New Data Streams**: Create and publish your own data streams directly from your Rust applications. Share data with other participants in the Data Layer, enabling collaboration and innovation.

# Installation

To use the Rust SDK for Data Layer in your project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
pubsub-rust = { git = "https://github.com/Synternet/pubsub-rust" }
```

# Getting Started

Before you begin using the Rust SDK, make sure you have the necessary credentials and access tokens from the [Synternet Developer Portal](https://portal.synternet.com/) platform. These credentials will allow you to connect to the Data Layer and subscribe to or publish data streams.

## Examples

For detailed usage examples, please refer to the [examples directory](https://github.com/Synternet/pubsub-rust/tree/main/examples) in the repository. These examples cover various scenarios and demonstrate how to utilize the SDK's features effectively.

The preferred method of authentication is using an access token from the developer portal.

Those examples demonstrate how to connect to a NATS server, subscribe to a subject, and publish messages to subject using the Synternet PubSub-Rust library.

## Contributing

We welcome contributions from the community! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/Synternet/pubsub-rust). We appreciate your feedback and collaboration in making this SDK even better.

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

If you encounter any difficulties or have questions regarding the Rust SDK for Data Layer, please reach out to our support team at [Discord #developers-chat](https://discord.com/channels/503896258881126401/1125658694399561738). We are here to assist you and ensure a smooth experience with our SDK.

We hope this documentation provides you with a comprehensive understanding of the Rust SDK for the Data Layer. Enjoy leveraging real-time data streams and unlocking the power of the Data Layer in your Rust applications!
