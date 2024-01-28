use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Presence {
    pub op: i32,
    pub d: PresenceData,
}

#[derive(Deserialize, Serialize)]
pub struct PresenceData {
    pub since: Option<String>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Activity {
    pub name: String,
    pub r#type: i32,
    pub details: Option<String>,
    pub timestamps : Option<ActivityTimestamps>,
    pub assets: Option<ActivityAssets>,
}

#[derive(Deserialize, Serialize)]
pub struct ActivityTimestamps {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}