use crate::models::{
    client::{Client, Event, EventCache, EventName, EventsType},
    gateway::Gateway,
};

impl Client {
    // Client constructor
    pub fn create(intents: i32) -> Client {
        let event_cache = EventCache { events: Vec::new() };

        Client {
            intents,
            event_cache,
            user: None,
            gateway: None
        }
    }

    // Client on event method
    pub fn on(&mut self, event: &str, callback: fn(data: EventsType)) {
        let event = Event {
            name: EventName(event.to_string()),
            callback,
        };

        self.event_cache.events.push(event);
    }

    // Client login method [start the wss connection]
    pub async fn login(&mut self, token: String) {
        let mut gateway = Gateway::init(self.intents, token).await;
        gateway.connect(self).await;
    }

    pub fn emit(&mut self, event_name: EventName, data: EventsType) {
        for event in &self.event_cache.events {
            if event.name == event_name {
                (event.callback)(data.clone());
            }
        }
    }
}
