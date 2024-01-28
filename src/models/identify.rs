use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Identify {
    pub op: i32,
    pub d: IdentifyData,
}

#[derive(Serialize, Deserialize)]
pub struct IdentifyData {
    pub intents: i32,
    pub token: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    pub os: String,
    pub browser: String,
    pub device: String,
}
