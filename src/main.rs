use futures_util::StreamExt;
use tokio::time::{Duration, interval};

use futures_util::SinkExt;
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use tokio::io::{AsyncBufReadExt, BufReader};

use std::env;

mod bot;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {


    let bot = bot::Bot{
        Name :String::from("fulika"),
        Token :env::var("TOKEN").expect("token does not exist"),
        Client :reqwest::Client::new()
    };
    println!("Client leady");

    let mut ws = bot::Bot::CreateWebSocket().await.unwrap();
    let (mut write, mut read) = ws.split();

    let value = bot.WaitMessage(&mut read).await.unwrap();

    let heartbeat_ms = value["d"]["heartbeat_interval"].as_u64().ok_or("heartbeat_intervalが見つからない").unwrap();

    bot.SendMessage("bot起動", "1286296768887259138").await;

    write.send(Message::Text(json!({
        "op": 2,
        "d": {
            "token": bot.Token,
            "intents": 33281,
            "properties": {
                "os": "windows",
                "browser": "fulika",
                "device": "fulika"
            }
        }
    }).to_string().into())).await.unwrap();

    let mut heartbeat = interval(Duration::from_millis(heartbeat_ms));

    let mut sequence:Option<u64> = value["s"].as_u64();

    loop {

        tokio::select! {

            _ = heartbeat.tick() => {
                println!("Heartbeat");
                write.send(
                    Message::Text(
                        json!({
                            "op": 1,
                            "d": sequence
                        }).to_string().into()
                    )
                ).await.unwrap();
            }

            message = bot.WaitMessage(&mut read) => {
                let value = message.unwrap();
                sequence = value["s"].as_u64();

                if (value["t"] == "MESSAGE_CREATE"){
                    if value["d"]["content"] == "!getcommit" {

                        let url = "https://api.github.com/repos/daihukuman/fulika/commits?per_page=1";

                        let response = bot.Client
                            .get(url)
                            .header("User-Agent", "MyApp")
                            .send()
                            .await?;

                        let commits: serde_json::Value = response.json().await?;

                        let latest = &commits[0];

                        let retstr = format!("## 最後のコミット\nHash: {}\nMessage: {}\nDate: {}\nこれまでの更新: https://github.com/daihukuman/fulika/commits/master/", latest["sha"], latest["commit"]["message"], latest["commit"]["author"]["date"]);
                        
                        bot.SendMessage(&retstr, value["d"]["channel_id"].as_str().unwrap()).await;
                    }
                }
            }
        }
    }

    

    Ok(())

}
