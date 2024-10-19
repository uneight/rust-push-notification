use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::config::firebase::Config;

const DEFAULT_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
const ACCESS_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug, Serialize, Deserialize)]
struct ServiceAccount {
    private_key: String,
    client_email: String,
    auth_uri: String,
    token_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    iss: String,  // issuer, typically your service account's email
    scope: String, // the scope of the API you're accessing (FCM)
    aud: String,   // audience, which is Google's OAuth token URL
    exp: usize,    // expiration time of the token
    iat: usize,    // issued at time
}

#[derive(Serialize, Deserialize)]
struct FcmNotification {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct FcmMessage<'a> {
    token: &'a str,
    notification: FcmNotification,
}

#[derive(Default)]
pub struct Fcm {
    cfg: Config,
}

impl Fcm {
    pub fn new(cfg: Config) -> Self {
        Fcm { cfg }
    }
    pub async fn send(&self, device_token: &str, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let fcm_token = access_token().await?;

        let client = reqwest::Client::new();
        let message = FcmMessage {
            token: device_token,
            notification: FcmNotification {
                title: title.to_string(),
                body: body.to_string(),
            },
        };

        let notification = serde_json::json!({
        "message": message,
    });

        client.post(&self.cfg.url)
            .bearer_auth(fcm_token)
            .json(&notification)
            .send()
            .await?;

        Ok(())
    }
}

async fn jwt_token(service_account_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let private_key = std::fs::read_to_string(service_account_key)?;

    let service_account = serde_json::from_str::<ServiceAccount>(&private_key)?;

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize;
    let exp = now + 3600;

    let claims = FirebaseClaims {
        iss: service_account.client_email.clone(), // Ensure proper ownership
        scope: DEFAULT_SCOPE.to_string(), // Correct FCM scope
        aud: ACCESS_TOKEN_URL.to_string(),
        exp,
        iat: now,
    };

    let header = Header {
        alg: Algorithm::RS256,
        ..Default::default()
    };

    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(service_account.private_key.as_bytes())?,
    )?;

    Ok(token)
}

async fn access_token() -> Result<String, Box<dyn std::error::Error>> {
    let token = jwt_token("service-account.json").await?;
    let client = reqwest::Client::new();
    let response = client
        .post(ACCESS_TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "assertion={}&grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer",
            token
        ))
        .send()
        .await?;

    let json_response: serde_json::Value = response.json().await?;

    if let Some(access_token) = json_response.get("access_token") {
        Ok(access_token.as_str().unwrap().to_string())
    } else {
        Err("Failed to obtain access token".into())
    }
}