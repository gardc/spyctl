use std::{io, net::SocketAddr, str::FromStr};

use ratatui::{prelude::CrosstermBackend, Terminal};
use spyctl::master::{
    comms::PeerMap,
    tui::{
        app::{App, AppResult, Log},
        event::{Event, EventHandler},
        handler::handle_key_events,
        tui::Tui,
    },
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new(PeerMap::default());
    // populate for testings sake
    app.add_log(Log::Info("Test :: Poop".to_string()));
    // app.add_slave(Slave {
    //     ip: "192.168.0.5".to_string(),
    //     sysinfo: SystemInfo {
    //         realname: "John Appleseed".to_string(),
    //         username: "johnappleseed".to_string(),
    //         langs: Some(vec!["en-us".to_string()]),
    //         devicename: "LAPTOPH1039".to_string(),
    //         hostname: Some("LTOP11".to_string()),
    //         platform: "Windows 11".to_string(),
    //         distro: "Windows 11".to_string(),
    //         desktop_env: "Windows 11".to_string(),
    //         arch: "Windows 11".to_string(),
    //     },
    // });
    // app.add_slave(Slave {
    //     ip: "192.168.0.10".to_string(),
    //     sysinfo: SystemInfo {
    //         realname: "Robert Frankberry".to_string(),
    //         username: "robertfrankberry".to_string(),
    //         langs: Some(vec!["en-us".to_string()]),
    //         devicename: "Lenovo Thinkpad X1 Carbon".to_string(),
    //         hostname: Some("RFRANK".to_string()),
    //         platform: "Windows 11".to_string(),
    //         distro: "Windows 11".to_string(),
    //         desktop_env: "Windows 11".to_string(),
    //         arch: "Windows 11".to_string(),
    //     },
    // });
    // app.add_slave(Slave {
    //     ip: "192.168.0.15".to_string(),
    //     sysinfo: SystemInfo {
    //         realname: "Jane Doe".to_string(),
    //         username: "janedoe".to_string(),
    //         langs: Some(vec!["en-us".to_string()]),
    //         devicename: "Macbook Pro 16".to_string(),
    //         hostname: Some("JDOE".to_string()),
    //         platform: "MacOS".to_string(),
    //         distro: "MacOS".to_string(),
    //         desktop_env: "MacOS".to_string(),
    //         arch: "MacOS".to_string(),
    //     },
    // });
    app.add_slave(&SocketAddr::from_str("192.168.0.2:6969").unwrap());
    app.add_slave(&SocketAddr::from_str("192.168.0.5:6969").unwrap());
    app.add_slave(&SocketAddr::from_str("192.168.0.10:6969").unwrap());

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
