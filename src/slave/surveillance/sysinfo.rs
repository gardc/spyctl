use std::fmt::{self, Display};

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

impl Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "    User's Name: '{}'", self.realname)?;
        writeln!(f, "    Username: '{}'", self.username)?;
        writeln!(f, "    User's Languages: {}", self.format_languages())?;
        writeln!(f, "    Device Name: '{}'", self.devicename)?;
        writeln!(
            f,
            "    Hostname: '{}'",
            self.hostname.as_deref().unwrap_or("Unknown")
        )?;
        writeln!(f, "    Platform: '{}'", self.platform)?;
        writeln!(f, "    Distribution: '{}'", self.distro)?;
        writeln!(f, "    Desktop Environment: '{}'", self.desktop_env)?;
        writeln!(f, "    Architecture: '{}'", self.arch)?;
        write!(f, "}}")
    }
}

impl SystemInfo {
    fn format_languages(&self) -> String {
        match &self.langs {
            Some(langs) if !langs.is_empty() => {
                let formatted_langs = langs
                    .iter()
                    .map(|lang| format!("        '{}'", lang))
                    .collect::<Vec<_>>()
                    .join(",\n");
                format!("{{\n{}\n    }}", formatted_langs)
            }
            _ => String::from("{}"),
        }
    }
}

pub fn get_system_info() -> SystemInfo {
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
        let machine_id = get_system_info();
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
