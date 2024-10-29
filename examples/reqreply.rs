extern crate pubsub_rust;
use async_nats::Request;
use bytes::Bytes;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "access-token";
    // https://docs.synternet.com/build/dl-access-points
    let broker_urls = "broker-eu-01.synternet.com";
    let stream_subject = "synternet.osmosis.rpc";
    let client = pubsub_rust::connect(broker_urls, access_token).await?;
    let json_payload = r#"{"endpoint":"header","params":{"height":"20121934"}}"#;
    let request_payload = Bytes::copy_from_slice(json_payload.as_bytes());
    let mut request = Request::default();
    request = request.payload(request_payload);
    let response = client
        .send_request(stream_subject.into(), request)
        .await
        .unwrap();
    println!("{:?}", response);
    Ok(())
}
