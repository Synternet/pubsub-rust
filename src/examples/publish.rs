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
