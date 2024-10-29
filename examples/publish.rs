extern crate pubsub_rust;

use bytes::Bytes;
use std::io::{self, Write};
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "access-token";
    // https://docs.synternet.com/build/dl-access-points
    let nats_server_ip = "broker-eu-01.synternet.com";
    let subject = String::from("example.subject");

    let client = pubsub_rust::connect(nats_server_ip, access_token).await?;

    let now = Instant::now();
    let data = Bytes::from("bar");
    for _ in 0..100 {
        print!("publishing\n");
        io::stdout().flush().unwrap();
        client.publish(subject.clone(), data.clone()).await?;
        sleep(Duration::from_secs(1)).await; // Wait for 1 second before the next publish
    }
    client.flush().await?;

    println!("published in {:?}", now.elapsed());

    Ok(())
}
