use models::client::{Client, EventsType};
use std::env;
use serde_json::to_string;
use tokio;
use crate::models::presence::{Activity, Presence, PresenceData};

mod models;
mod modules;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let mut client = Client::create(122879);

    client.on("ready", |data: EventsType| match data {

        // bot parameter = Clone of client.user
        EventsType::Ready(bot) => {
            let botdata = bot.unwrap();
            let botuser = botdata.user.clone().unwrap();


            let presence = Presence {
                op: 3,
                d: PresenceData {
                    since: None,
                    status: String::from("online"),
                    afk: false,
                    url: None,
                    activities: vec![Activity {
                        name: format!("{}", botuser.username),
                        r#type: 0,
                        assets: None,
                        details: None,
                        timestamps: None,
                    }],
                },
            };

            let presence = to_string(&presence).unwrap();
            println!("{}", presence);

            botdata.set_presence(presence);
        }
    });

    client.login(env::var("TOKEN").unwrap()).await;
}
