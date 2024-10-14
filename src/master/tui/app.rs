use std::{error, net::SocketAddr};

use ratatui::widgets::ListState;

use crate::{
    common::messages::{Command, Response},
    master::comms::{send_command_to_slave, PeerMap},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum AppState {
    SlaveList,
    SlaveMenu,
}

#[derive(Debug)]
pub enum IncomingMessage {
    NewConnection(SocketAddr),
    NewResponse(SocketAddr, Response),
    LogResult(String),
    LogInfo(String),
    LogError(String),
}

#[derive(Debug)]
pub enum Log {
    Result(String),
    Info(String),
    Error(String),
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,

    pub peer_map: PeerMap,
    pub state: AppState,
    pub slaves: Vec<SocketAddr>,
    pub slave_list_state: ListState,
    pub slave_menu_state: ListState,
    pub logs: Vec<Log>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            peer_map: PeerMap::default(),
            slaves: Vec::new(),
            slave_list_state: ListState::default(),
            slave_menu_state: ListState::default(),
            logs: Vec::new(),
            state: AppState::SlaveList,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(peer_map: PeerMap) -> Self {
        Self {
            running: true,
            slaves: Vec::new(),
            slave_list_state: ListState::default(),
            slave_menu_state: ListState::default(),
            logs: Vec::new(),
            state: AppState::SlaveList,
            peer_map, // TODO: this should use channels to communicate with the comms manager instead of this peer map
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_slave(&mut self, client: &SocketAddr) {
        self.slaves.push(client.clone());
    }

    pub fn remove_slave(&mut self, ip: &SocketAddr) {
        self.slaves.retain(|client| client != ip);
    }

    pub fn next(&mut self) {
        match self.state {
            AppState::SlaveList => {
                let i = match self.slave_list_state.selected() {
                    Some(i) => {
                        if i >= self.slaves.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.slave_list_state.select(Some(i));
            }
            AppState::SlaveMenu => {
                let i = match self.slave_menu_state.selected() {
                    Some(i) => {
                        if i >= 1 {
                            // Assuming 2 options in slave menu
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.slave_menu_state.select(Some(i));
            }
        }
    }

    pub fn previous(&mut self) {
        match self.state {
            AppState::SlaveList => {
                let i = match self.slave_list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.slaves.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.slave_list_state.select(Some(i));
            }
            AppState::SlaveMenu => {
                let i = match self.slave_menu_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            1 // Assuming 2 options in slave menu
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.slave_menu_state.select(Some(i));
            }
        }
    }

    pub fn select(&mut self) {
        match self.state {
            AppState::SlaveList => {
                if self.slave_list_state.selected().is_some() {
                    self.state = AppState::SlaveMenu;
                    self.slave_menu_state.select(Some(0));
                }
            }
            AppState::SlaveMenu => {
                if let Some(selected) = self.slave_menu_state.selected() {
                    match selected {
                        0 => self.take_screenshot(),
                        1 => self.show_system_info(),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn back(&mut self) {
        match self.state {
            AppState::SlaveList => {} // Already at the top level, do nothing
            AppState::SlaveMenu => {
                self.state = AppState::SlaveList;
                self.slave_menu_state.select(None);
            }
        }
    }

    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
    }

    fn take_screenshot(&mut self) {
        if let Some(selected_slave) = self.slave_list_state.selected() {
            if let Some(slave_addr) = self.slaves.get(selected_slave) {
                if send_command_to_slave(Command::CaptureScreens, *slave_addr, &self.peer_map) {
                    self.add_log(Log::Info(format!(
                        "Requested screenshot from {}",
                        slave_addr
                    )));
                } else {
                    self.add_log(Log::Error(format!(
                        "Failed to send screenshot request to {}",
                        slave_addr
                    )));
                }
            } else {
                self.add_log(Log::Error("No slave selected".to_string()));
            }
        } else {
            self.add_log(Log::Error("No slave selected".to_string()));
        }
    }

    fn show_system_info(&mut self) {
        if let Some(selected_slave) = self.slave_list_state.selected() {
            if let Some(slave_addr) = self.slaves.get(selected_slave) {
                if send_command_to_slave(Command::GetSystemInfo, *slave_addr, &self.peer_map) {
                    self.add_log(Log::Info(format!(
                        "Requested system info from {}",
                        slave_addr
                    )));
                } else {
                    self.add_log(Log::Error(format!(
                        "Failed to send system info request to {}",
                        slave_addr
                    )));
                }
            } else {
                self.add_log(Log::Error("No slave selected".to_string()));
            }
        } else {
            self.add_log(Log::Error("No slave selected".to_string()));
        }
    }
}
