use std::thread;

use spyctl::{
    common::messages::Command,
    master::comms::{run_master, send_command_to_all, PeerMap},
};

#[tokio::main]
pub async fn main() {
    let addr = "localhost:6969";

    let peer_map = run_master(addr).await.unwrap();

    // Start the TUI in a separate thread
    start_tui(peer_map.clone());

    // Keep the main thread alive
    tokio::signal::ctrl_c().await.unwrap();
    println!("Shutting down");
}

fn start_tui(peer_map: PeerMap) {
    thread::spawn(move || {
        // Your TUI logic here
        loop {
            // Example: send a command to all slaves every 5 seconds
            send_command_to_all(Command::CaptureScreens, &peer_map);
            thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}
