use serde::{Deserialize, Serialize};
use crate::models::gateway::Gateway;

#[derive(Clone)]
pub struct EventName(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub intents: i32,
    pub event_cache: EventCache,
    pub user: Option<BotData>,
    pub gateway: Option<Gateway>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotData {
    pub avatar: Option<String>,
    pub bot: bool,
    pub discriminator: Option<String>,
    pub email: Option<String>,
    pub flags: i32,
    pub id: String,
    pub username: String,
    pub verified: bool,
}

impl Client {
    pub fn set_presence(self, presence: String) {
        self.gateway.unwrap().send_data(presence);
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct EventCache {
    pub events: Vec<Event>,
}

#[derive(Clone)]
pub enum EventsType {
    Ready(Option<Client>),
}

type Callback = fn(data: EventsType);

#[derive(Clone)]
pub struct Event {
    pub name: EventName,
    pub callback: Callback,
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;

        let callback: Callback = |_| {};

        Ok(Event {
            name: EventName(name),
            callback,
        })
    }
}

impl PartialEq for EventName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Event> for EventName {
    fn eq(&self, other: &Event) -> bool {
        &self.0 == &other.name.0
    }
}

impl PartialEq<EventName> for Event {
    fn eq(&self, other: &EventName) -> bool {
        &self.name.0 == &other.0
    }
}

impl PartialEq<String> for EventName {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<EventName> for String {
    fn eq(&self, other: &EventName) -> bool {
        self == &other.0
    }
}
