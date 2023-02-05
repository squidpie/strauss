/*
main.rs
Copyright (C) 2023
Squidpie
*/

//! Strauss Chat uService
//!
//! Translates Twitch chat messages to Redis PubSub network as JSON Strings.
//!
//! [unimplemented] Provides interface for sending chat messages

use std::collections::HashMap;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use serde_yaml;

use strausslib::redisclient::RedisClient;
use strausslib::twitch::chat::STRAUSS_CHAT_MSG_REDIS_CH;

#[derive(Debug)]
struct ChatService<R> {
    conf: ChatConfig,
    redis: R,
}

impl ChatService<RedisClient> {
    /// ***blocking***
    ///
    /// twitch to redis translation
    pub async fn run(&mut self) {
        let config = ClientConfig::default();
        let (mut incoming_msg, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        client.join(self.conf.channel.to_owned()).unwrap();

        while let Some(message) = incoming_msg.recv().await {
            let msg: Option<String> = match message {
                ServerMessage::Privmsg(priv_msg) => Some(serde_json::to_string(&priv_msg).unwrap()),
                _ => None,
            };
            if let Some(json) = msg {
                self.redis
                    .publish(STRAUSS_CHAT_MSG_REDIS_CH, &json)
                    .expect("<chat>::ERROR::FAILED TO PUBLISH")
            }
        }
    }
}

#[derive(Debug, Default)]
struct ChatConfig {
    channel: String,
}

impl ChatConfig {
    pub async fn read(mut path: String) -> Result<Self, Box<dyn std::error::Error>> {
        path.push_str("/strauss.yml");
        let f = std::fs::File::open(path).expect("<chat>::ERROR::UNABLE TO OPEN CONFIG");
        let c: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_reader(f).expect("<chat>::ERROR::SERDE_YAML FAILURE");
        let config = c
            .get_key_value("chat")
            .expect("<chat>::ERROR::NO CHAT CONFIG SPECIFIED");
        Ok(ChatConfig {
            channel: config
                .1
                .get("channel")
                .expect("<chat>::ERROR::NO CHANNEL DEFINED")
                .to_owned(),
        })
    }
}

/// Main
///
/// Requires \[optional arg1\]/strauss.yml
#[tokio::main]
pub async fn main() {
    let mut path = "".to_owned();
    if let Some(arg) = std::env::args().nth(1) {
        path = arg
    }
    let conf = ChatConfig::read(path.to_owned()).await.unwrap();
    let mut chat = ChatService {
        conf: conf,
        redis: RedisClient::new(),
    };
    chat.run().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    use strausslib::redisclient::mockredisclient::RedisClient;

    #[tokio::test]
    async fn chat_main() {
        let conf = ChatConfig::default();
        let mut _dut = ChatService {
            conf: conf,
            redis: RedisClient::new(),
        };
    }
}
