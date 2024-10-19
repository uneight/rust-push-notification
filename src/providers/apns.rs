use std::fs;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::apns::{Config};

const APNS_PRIVATE_KEY_NAME: &str = "apns-private-key.p8";

#[derive(Serialize, Deserialize)]
struct ApnsClaims {
    iss: String,
    iat: i64,
}

#[derive(Default)]
pub struct Apns {
    cfg: Config,
}

impl Apns {
    pub fn new(cfg: Config) -> Self {
        Apns { cfg }
    }
    fn generate_apns_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let private_key = fs::read_to_string(APNS_PRIVATE_KEY_NAME)?;

        let now = Utc::now().timestamp();

        let mut header = Header {
            alg: Algorithm::ES256,
            ..Default::default()
        };

        header.kid = Some(self.cfg.key_id.to_owned());
        header.typ = None;

        let claims = ApnsClaims {
            iss: self.cfg.team_id.to_owned(),
            iat: now,
        };

        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_ec_pem(private_key.as_bytes())?,
        )?;

        Ok(token)
    }

    pub async fn send(&self, device_token: &str, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.generate_apns_token()?;

        let client = Client::builder()
            .http2_prior_knowledge()
            .build()?;

        let payload = json!({
        "aps": {
            "alert": {
                "title": title,
                "body": body,
            },
            "sound": "default",
        }
    });

        let url = format!("{}{}", self.cfg.url, device_token);

        let res = client
            .post(&url)
            .header("authorization", format!("Bearer {token}"))
            .header("apns-push-type", "alert")
            .header("apns-topic", self.cfg.bundle_id.to_owned())
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", res.status());

        if res.status().is_success() {
            println!("Notification sent successfully!");
        } else {
            eprintln!("Failed to send notification. Status: {:?}", res.status());
            let response_body = res.text().await?;
            eprintln!("Response body: {}", response_body);
        }

        Ok(())
    }
}