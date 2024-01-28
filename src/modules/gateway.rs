use futures_util::{SinkExt, StreamExt};
use serde_json::to_string;
use tokio_tungstenite::tungstenite::Message;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;

use crate::models::client::{BotData, Client, EventName};
use crate::models::gateway::Gateway;
use crate::models::identify::{Identify, IdentifyData, Properties};

// Gateway implementation
impl Gateway {

    // Gateway constructor
    pub async fn init(intents: i32, token: String) -> Gateway {
        let gate_url = String::from("wss://gateway.discord.gg/?v=10&encoding=json");

        let (stream, _) = connect_async(&gate_url).await.unwrap();
        let (mut write, mut read) = stream.split();

        let identify = Identify {
            op: 2,
            d: IdentifyData {
                intents,
                token: token.clone(),
                properties: Properties {
                    os: String::from("linux"),
                    browser: String::from("rusty"),
                    device: String::from("rusty"),
                },
            },
        };

        let message = to_string(&identify).unwrap();

        write.send(Message::Text(message)).await.unwrap();

        Gateway {
            url: gate_url,
            intents,
            write: Some(Arc::new(Mutex::new(write))),
            read: Some(Arc::new(Mutex::new(read))),
            token,
        }
    }

    pub fn send_data(self, data: String) {
        println!("Sending data to gateway");


        let write = Arc::clone(&self.write.clone().unwrap());

        tokio::spawn(async move {
            write.lock().await.send(Message::Text(data)).await.unwrap();
        });
    }

    // Gateway connect method
    pub async fn connect(&mut self, client: &mut Client) {
        client.gateway = Some(self.clone());

        // while the connection is open
        while let Some(msg) = self.read.clone().unwrap().lock().await.next().await {
            let message = msg.unwrap();

            let parsed = serde_json::from_str::<serde_json::Value>(&message.to_string());

            if parsed.is_err() {
                println!("Error parsing message: {}", parsed.err().unwrap());
                continue;
            }

            let parsed = parsed.unwrap();
            let op = parsed["op"].as_i64().unwrap();

            match op {
                // 10 = EVENT Hello
                10 => {
                    println!("Received hello event from gateway 🎉");

                    let interval = parsed["d"]["heartbeat_interval"].as_i64().unwrap();
                    let interval = std::time::Duration::from_millis(interval as u64);

                    let mut interval = tokio::time::interval(interval);

                    let writer = Arc::clone(&self.write.clone().unwrap());

                    tokio::spawn(async move {
                        loop {
                            interval.tick().await;
                            writer.lock().await.send(Message::Text(String::from("{\"op\": 1, \"d\": null}")))
                              .await.unwrap();
                        }
                    });

                }

                // 0 = EVENT Dispatch
                0 => {
                    let event_name = parsed["t"].as_str().unwrap().to_string();
                    let event_data = parsed["d"].clone();

                    match event_name.as_str() {
                        "READY" => {
                            if let Some(bot) = event_data["user"].as_object() {
                                client.user = Some(BotData {
                                    id: bot["id"].as_str().unwrap().to_string(),
                                    username: bot["username"].as_str().unwrap().to_string(),
                                    discriminator: bot["discriminator"].as_str().map(|s| s.to_string()),
                                    avatar: bot["avatar"].as_str().map(|s| s.to_string()),
                                    bot: bot["bot"].as_bool().unwrap(),
                                    email: bot["email"].as_str().map(|s| s.to_string()),
                                    flags: bot["flags"].as_i64().unwrap() as i32,
                                    verified: bot["verified"].as_bool().unwrap(),
                                });

                                client.emit(
                                    EventName("ready".to_string()),
                                    crate::models::client::EventsType::Ready(Some(client.clone())),
                                );
                            }
                        }
                        _ => println!("Received event: {}", event_name),
                    }
                }
                _ => println!("Code: {}", op),
            }
        }
    }
}
