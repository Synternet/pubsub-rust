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