/*
config.rs
Copyright (C) 2023
Squidpie
*/

//! Twitch Config
//! Utils for storing Twitch credentials and channel info
//!
//! Channel derived from strauss.yml
//! chat:
//!     channel: <channel name>
//! TODO: Change hardcoded strauss.yml to config arg/env
//!
//! Credentials derived from environment variables
//! TWITCH_USER <login user name>
//! TWITCH_TOKEN <oauth token>

use crate::config::StraussConfig;


#[derive(Debug, Clone)]
struct TwitchChannel {
    name: String,
}

impl TwitchChannel {
    fn load(config: StraussConfig) -> Self {
        let name = config.services["chat"]["channel"].to_owned();
        TwitchChannel { name }
    }
}

#[derive(Clone)]
struct TwitchCredentials {
    login_name: String,
    oauth_token: String,
}

impl std::fmt::Debug for TwitchCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let replace = String::from("***");
        f.debug_struct("TwitchCredentials")
            .field("login_name", &replace)
            .field("oauth_token", &replace)
            .finish()
    }
}

impl TwitchCredentials {
    fn load() -> Self {
        Self {
            login_name: std::env::var("TWITCH_USER")
                .expect("<TwitchCredentials>::ERROR::TWITCH_USER .secrets UNDEFINED"),
            oauth_token: std::env::var("TWITCH_TOKEN")
                .expect("<TwitchCredentials>::ERROR::TWITCH_TOKEN .secrets UNDEFINED"),
        }
    }
}


#[derive(Debug, Clone)]
pub struct TwitchConfig {
    channel: TwitchChannel,
    credentials: TwitchCredentials,
}

impl TwitchConfig {
    pub fn load(config: StraussConfig ) -> Self {
        TwitchConfig {
            channel: TwitchChannel::load(config),
            credentials: TwitchCredentials::load(),
        }
    }
}

pub trait Channel {
    fn channel(&self) -> String;
}

pub trait Credentials {
    fn credentials(&self) -> (String, String);
}
pub trait Config: Channel + Credentials {}

impl Channel for TwitchConfig {
    fn channel(&self) -> String {
        self.channel.name.clone()
    }
}

impl Credentials for TwitchConfig {
    fn credentials(&self) -> (String, String) {
        (
            self.credentials.login_name.clone(),
            self.credentials.oauth_token.clone(),
        )
    }
}

impl Config for TwitchConfig {}

impl std::fmt::Debug for Box<dyn Config> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Config: Channel + Credentials {}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn read_channel() {
       let mut chat_config : HashMap<String, String> = HashMap::new();
       let mut services : HashMap<String, HashMap<String, String>> = HashMap::new();
       chat_config.insert(String::from("channel"), String::from("test_channel"));
       services.insert(String::from("chat"), chat_config);
       let config = StraussConfig{services};

        let dut = TwitchChannel::load(config);
        assert_eq!(dut.name, "test_channel");
        println!("::TEST OUTPUT:: {dut:#?}")
    }

    #[test]
    fn read_credentials() {
        std::env::set_var("TWITCH_USER", "test_user");
        std::env::set_var("TWITCH_TOKEN", "0xdeadbeef");
        
        let dut = TwitchCredentials::load();
        assert_eq!(dut.login_name, "test_user");
        assert_eq!(dut.oauth_token, "0xdeadbeef");
        println!("::TEST OUTPUT:: {dut:#?}")
    }

    #[test]
    fn create_config() {
        std::env::set_var("TWITCH_USER", "test_user");
        std::env::set_var("TWITCH_TOKEN", "0xdeadbeef");
        
        let mut services: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut chat_config: HashMap<String, String> = HashMap::new();
        chat_config.insert(String::from("channel"), String::from("test_channel"));
        services.insert(String::from("chat"), chat_config);
        let config = StraussConfig{services};

        let dut = TwitchConfig::load(config);
        assert_eq!(dut.channel(), "test_channel");
        assert_eq!(dut.credentials(), (String::from("test_user"), String::from("0xdeadbeef")));
        assert_eq!(dut.credentials.login_name, "test_user");
        assert_eq!(dut.credentials.oauth_token, "0xdeadbeef");
        println!("::TEST OUTPUT:: {dut:#?}")
    }
}
