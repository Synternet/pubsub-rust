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
