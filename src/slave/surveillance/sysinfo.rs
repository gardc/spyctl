use serde::{Deserialize, Serialize};
use whoami::fallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub realname: String,
    pub username: String,
    pub langs: Option<Vec<String>>,
    pub devicename: String,
    pub hostname: Option<String>,
    pub platform: String,
    pub distro: String,
    pub desktop_env: String,
    pub arch: String,
}

pub fn get_machine_id() -> SystemInfo {
    SystemInfo {
        realname: whoami::realname(),
        username: whoami::username(),
        langs: whoami::langs()
            .ok()
            .map(|langs| langs.map(|l| l.to_string()).collect()),
        devicename: whoami::devicename(),
        hostname: fallible::hostname().ok(),
        platform: whoami::platform().to_string(),
        distro: whoami::distro(),
        desktop_env: whoami::desktop_env().to_string(),
        arch: whoami::arch().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_whoami() {
        let machine_id = get_machine_id();
        assert!(machine_id.realname.len() > 0);
        println!("Machine ID Information:");
        println!("User's Name:           {}", machine_id.realname);
        println!("User's Username:       {}", machine_id.username);
        println!("User's Language:       {:?}", machine_id.langs.unwrap());
        println!("Device's Pretty Name:  {}", machine_id.devicename);
        println!("Device's Hostname:     {}", machine_id.hostname.unwrap());
        println!("Device's Platform:     {}", machine_id.platform);
        println!("Device's OS Distro:    {}", machine_id.distro);
        println!("Device's Desktop Env.: {}", machine_id.desktop_env);
        println!("Device's CPU Arch:     {}", machine_id.arch);
    }
}
