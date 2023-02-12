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

use crate::config::SerialRead;

#[derive(Debug, Clone)]
struct TwitchChannel {
    channel: String,
}

impl TwitchChannel {
    fn load(yml_reader: Box<dyn SerialRead>) -> Self {
        let yml = yml_reader.read("strauss.yml".to_owned());
        TwitchChannel {
            channel: yml.get("chat").unwrap().get("channel").unwrap().to_owned(),
        }
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
    channel: String,
    credentials: TwitchCredentials,
}

impl TwitchConfig {
    pub fn load(yml_reader: Box<dyn SerialRead>) -> Self {
        TwitchConfig {
            channel: TwitchChannel::load(yml_reader).channel,
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
        self.channel.clone()
    }
}

impl Credentials for TwitchConfig {
    fn credentials(&self) -> (String, String) {
        (self.credentials.login_name.clone(), self.credentials.oauth_token.clone())
    }
}

impl Config for TwitchConfig{}

impl std::fmt::Debug for Box<dyn Config> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Config: Channel + Credentials {}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::MockSerialRead;
    use std::collections::HashMap;

    #[test]
    fn read_channel() {
        let mut yml_reader = MockSerialRead::default();
        let mut test_config: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut chat_entry: HashMap<String, String> = HashMap::new();
        chat_entry.insert(String::from("channel"), String::from("test_channel"));
        test_config.insert(String::from("chat"), chat_entry);
        yml_reader
            .expect_read()
            .once()
            .returning(move |_| test_config.to_owned());
        let dut = TwitchChannel::load(Box::new(yml_reader));
        assert_eq!(dut.channel, "test_channel");
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
        // Mock File I/O
        let mut yml_reader = MockSerialRead::default();
        let mut test_config: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut chat_entry: HashMap<String, String> = HashMap::new();
        chat_entry.insert(String::from("channel"), String::from("test_channel"));
        test_config.insert(String::from("chat"), chat_entry);
        yml_reader
            .expect_read()
            .once()
            .returning(move |_| test_config.to_owned());
        let yml_reader = Box::new(yml_reader);

        // Setup ENV Vars
        std::env::set_var("TWITCH_USER", "test_user");
        std::env::set_var("TWITCH_TOKEN", "0xdeadbeef");

        let dut = TwitchConfig::load(yml_reader);
        assert_eq!(dut.channel, "test_channel");
        assert_eq!(dut.credentials.login_name, "test_user");
        assert_eq!(dut.credentials.oauth_token, "0xdeadbeef");
        println!("::TEST OUTPUT:: {dut:#?}")
    }
}
