use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[derive(Deserialize, Serialize, Clone)]
pub struct Gateway {
    pub url: String,
    pub intents: i32,
    pub token: String,
    #[serde(skip)]
    pub write: Option<Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>>,
    #[serde(skip)]
    pub read: Option<Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>>,
}
