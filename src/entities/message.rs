use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
#[derive(Deserialize, Debug)]
pub struct Message {
    pub topic: String,
    pub title: String,
    pub body: String,
    pub data: HashMap<String, Value>,
}