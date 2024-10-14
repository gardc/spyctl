use serde::{Deserialize, Serialize};

use crate::slave::surveillance::{screen::Screenshot, sysinfo::SystemInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    CaptureScreens,
    GetSystemInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Screens(Vec<Screenshot>),
    SystemInfo(SystemInfo),
    Status(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Command(Command),
    Response(Response),
}

pub fn create_command(cmd: Command) -> Message {
    Message::Command(cmd)
}

pub fn create_response(resp: Response) -> Message {
    Message::Response(resp)
}
