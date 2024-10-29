extern crate async_nats;
extern crate nkeys;
extern crate rand;
extern crate serde;

use data_encoding::BASE64URL_NOPAD;
use rand::Rng;
use serde::Serialize;
use serde_json::to_string;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Serialize)]
struct Payload {
    jti: String,
    iat: i64,
    iss: String,
    name: String,
    sub: String,
    nats: HashMap<String, Value>,
}

fn generate_jti() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random_number: f64 = rand::thread_rng().gen();
    format!(
        "{}{}",
        now,
        random_number.to_string().split('.').nth(1).unwrap()
    )
}

fn generate_iat() -> i64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now as i64
}

fn get_nats_config() -> HashMap<String, serde_json::Value> {
    let mut config: HashMap<String, serde_json::Value> = HashMap::new();

    // existing entries
    config.insert("subs".to_string(), serde_json::json!(-1));
    config.insert("data".to_string(), serde_json::json!(-1));
    config.insert("payload".to_string(), serde_json::json!(-1));
    config.insert("version".to_string(), serde_json::json!(2));

    // new entries
    config.insert(
        "pub".to_string(),
        serde_json::json!(HashMap::<String, i64>::new()),
    );
    config.insert(
        "sub".to_string(),
        serde_json::json!(HashMap::<String, i64>::new()),
    );
    config.insert("type".to_string(), serde_json::json!("user"));

    config
}

fn sign_jwt(
    payload: &Payload,
    account: &nkeys::KeyPair,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut header = HashMap::new();
    header.insert("typ".to_string(), "JWT".to_string());
    header.insert("alg".to_string(), "ed25519-nkey".to_string());

    let header_bytes = to_string(&header)?;
    let header_encoded = BASE64URL_NOPAD.encode(header_bytes.as_bytes());

    let payload_bytes = to_string(&payload)?;
    let payload_encoded = BASE64URL_NOPAD.encode(payload_bytes.as_bytes());

    let jwtbase = format!("{}.{}", header_encoded, payload_encoded);

    let signature_bytes = account.sign(jwtbase.as_bytes())?;
    let signature = BASE64URL_NOPAD.encode(&signature_bytes);

    Ok(format!("{}.{}", jwtbase, signature))
}

pub fn create_app_jwt(seed: &str) -> Result<String, Box<dyn std::error::Error>> {
    let account = nkeys::KeyPair::from_seed(seed)?;

    let acc_pub_key = account.public_key();

    let payload = Payload {
        jti: generate_jti(),
        iat: generate_iat(),
        iss: acc_pub_key.clone(),
        name: "developer".to_string(),
        sub: acc_pub_key,
        nats: get_nats_config(),
    };

    let jwt = sign_jwt(&payload, &account)?;

    let creds = format!(
        "-----BEGIN NATS USER JWT-----\n{}\n------END NATS USER JWT------\n\n\
                         ************************* IMPORTANT *************************\n\
                         NKEY Seed printed below can be used to sign and prove identity.\n\
                         NKEYs are sensitive and should be treated as secrets.\n\n\
                         -----BEGIN USER NKEY SEED-----\n{}\n------END USER NKEY SEED------\n\n\
                         *************************************************************",
        jwt, seed
    );
    Ok(creds)
}

pub async fn connect(
    nats_server_ip: &str,
    seed: &str,
) -> Result<async_nats::Client, Box<dyn std::error::Error>> {
    let creds = create_app_jwt(seed)?;
    let client = async_nats::ConnectOptions::with_credentials(&creds)
        .expect("failed to parse creds")
        .connect(nats_server_ip)
        .await?;
    Ok::<_, Box<dyn std::error::Error>>(client)
}
