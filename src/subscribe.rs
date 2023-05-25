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