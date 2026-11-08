use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use reqwest::StatusCode;
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    MaybeTlsStream,
    WebSocketStream,
};
use tokio_tungstenite::tungstenite::Message;

pub struct Bot{
    pub Name :String,
    pub Token :String,
    pub Client :reqwest::Client
}

impl Bot{
    pub async fn CreateWebSocket() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn std::error::Error>> {
        let (mut ws, _) = connect_async(
            "wss://gateway.discord.gg/?v=10&encoding=json"
        ).await?;

        Ok(ws)
    }
    pub async fn WaitMessage(&self, read :&mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let value :serde_json::Value = serde_json::from_str(
            read.next().await.ok_or("接続終了")??.to_text()?
        )?;

        Ok(value)
    }

    pub async fn SendMessage(&self, message : &str, channel_id : &str) -> StatusCode{

        let response = self.Client
        .post(format!(
            "https://discord.com/api/v10/channels/{}/messages",
            channel_id
        ))
        .header("Authorization", format!("Bot {}", self.Token))
        .json(&json!({
            "content": message
        }))
        .send()
        .await
        .unwrap();

        response.status()

    }

}