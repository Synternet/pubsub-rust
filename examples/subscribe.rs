extern crate pubsub_rust;
use futures::stream::StreamExt;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "access-token";
    // https://docs.synternet.com/build/dl-access-points
    let nats_server_ip = "broker-eu-01.synternet.com";
    let subscribe_subject = "synternet.ethereum.tx";
    let client = pubsub_rust::connect(nats_server_ip, access_token).await?;
    let mut subscriber = client.subscribe(subscribe_subject.into()).await.unwrap();
    while let Some(message) = subscriber.next().await {
        println!("{:?}", message);
    }
    Ok(())
}
