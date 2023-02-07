/*
main.rs
Copyright (C) 2023
Squidpie
*/

//! Strauss Chat uService
//!
//! Transports Twitch chat messages to Redis strauss-chat-msg-rx as JSON Strings.
//!
//! Transports Redis strauss-chat-msg-tx messages to Twitch Chat

use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use tokio::signal;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use strausslib::redisclient::RedisClient;
use strausslib::twitch::chat::STRAUSS_CHAT_MSG_RX_REDIS_CH;
use strausslib::twitch::chat::STRAUSS_CHAT_MSG_TX_REDIS_CH;

#[derive(Debug)]
struct ChatService<R> {
    conf: ChatConfig,
    redis: R,
}

impl Clone for ChatService<RedisClient> {
    fn clone(&self) -> Self {
        ChatService {
            conf: self.conf.clone(),
            redis: RedisClient::default(),
        }
    }
}

impl ChatService<RedisClient> {
    /// ***blocking***
    ///
    /// twitch to redis transport
    pub fn listen_twitch(&mut self) -> impl Future<Output = ()> {
        let mut selfc = self.clone();
        async move {
            let login_name = std::env::var("TWITCH_USER")
                .expect("<ChatService>::ERROR::TWITCH_USER .secrets UNDEFINED");
            let oauth_token = std::env::var("TWITCH_TOKEN")
                .expect("<ChatService>::ERROR::TWITCH_TOKEN .secrets UNDEFINED");

            let config = ClientConfig::new_simple(StaticLoginCredentials::new(
                login_name,
                Some(oauth_token),
            ));

            let (mut incoming_msg, client): (
                tokio::sync::mpsc::UnboundedReceiver<ServerMessage>,
                TwitchIRCClient<
                    twitch_irc::transport::tcp::TCPTransport<twitch_irc::transport::tcp::TLS>,
                    StaticLoginCredentials,
                >,
            ) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
            client.join(selfc.conf.channel.to_owned()).unwrap();
            println!("<ChatService>::INFO::Twitch->Redis Transport Client Connected to Twitch");
            println!("<ChatService>::INFO::Twitch->Redis Transport Client Listening...");
            while let Some(message) = incoming_msg.recv().await {
                let msg: Option<String> = match message {
                    ServerMessage::Privmsg(priv_msg) => {
                        Some(serde_json::to_string(&priv_msg).unwrap())
                    }
                    _ => None,
                };
                if let Some(json) = msg {
                    selfc
                        .redis
                        .publish(STRAUSS_CHAT_MSG_RX_REDIS_CH, &json)
                        .expect("<ChatService>::ERROR::FAILED TO PUBLISH")
                }
            }
        }
    }
    /// ***blocking***
    ///
    /// redis to twitch transport
    pub fn listen_redis(&self) -> impl Future<Output = ()> {
        let mut selfc = self.clone();
        async move {
            let login_name = std::env::var("TWITCH_USER")
                .expect("<ChatService>::ERROR::TWITCH_USER .secret UNDEFINED");
            let oauth_token = std::env::var("TWITCH_TOKEN")
                .expect("<ChatService>::ERROR::TWITCH_TOKEN .secret UNDEFINED");

            let config = ClientConfig::new_simple(StaticLoginCredentials::new(
                login_name,
                Some(oauth_token),
            ));

            let (_, client): (
                tokio::sync::mpsc::UnboundedReceiver<ServerMessage>,
                TwitchIRCClient<
                    twitch_irc::transport::tcp::TCPTransport<twitch_irc::transport::tcp::TLS>,
                    StaticLoginCredentials,
                >,
            ) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
            client
                .join(selfc.conf.channel.to_owned())
                .expect("<ChatService>::ERROR::TWITCH CLIENT JOIN FAILED");
            println!("<ChatService>::INFO::Redis->Twitch Transport Client Connected to Twitch");
            let mut pubsub = selfc
                .redis
                .pubsub(STRAUSS_CHAT_MSG_TX_REDIS_CH)
                .await
                .expect("<ChatService>::ERROR::FAILED TO GET REDIS PUBSUB");
            println!("<ChatService>::INFO::Redis->Twitch Transport Client Subscribed to {STRAUSS_CHAT_MSG_TX_REDIS_CH}");
            println!("<ChatService>::INFO::Redis->Twitch Transport Client Listening...");
            loop {
                let msg = pubsub
                    .get_message()
                    .expect("<ChatService>::ERROR::FAILED TO LOAD MSG");
                client
                    .say(
                        selfc.conf.channel.to_owned(),
                        msg.get_payload()
                            .expect("<ChatService>::ERROR::FAILED TO LOAD MSG PAYLOAD"),
                    )
                    .await
                    .expect("<ChatService>::ERROR::TWITCH SAY FAILED");
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct ChatConfig {
    channel: String,
}

impl ChatConfig {
    pub fn read() -> Result<Self, Box<dyn Error>> {
        let path = String::from("strauss.yml");
        let f = std::fs::File::open(path).expect("<ChatService>::ERROR::UNABLE TO OPEN CONFIG");
        let config: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_reader(f).expect("<ChatService>::ERROR::SERDE_YAML FAILURE");
        let config = config
            .get_key_value("chat")
            .expect("<ChatService>::ERROR::NO CHAT CONFIG SPECIFIED")
            .1;
        let config = ChatConfig {
            channel: config
                .get("channel")
                .expect("<ChatService>::ERROR::NO CHANNEL DEFINED")
                .to_owned(),
        };
        println!("<ChatService>::INFO::Chat Service Config Loaded {config:?}");
        Ok(config)
    }
}

/// Main
///
#[tokio::main]
pub async fn main() {
    let conf =
        ChatConfig::read().expect("<ChatService>::ERROR::CREATE CONFIG FAILED");
    let mut chat: ChatService<RedisClient> = ChatService {
        conf,
        redis: RedisClient::default(),
    };

    let t = tokio::spawn(chat.listen_twitch());
    let r = tokio::spawn(chat.listen_redis());

    signal::ctrl_c()
        .await
        .expect("<ChatService>::ERROR::CTRL-C FAILED");
    r.abort();
    t.abort();

    //r.await.expect("<ChatService>::ERROR::REDIS LISTEN FAILED");
    //t.await.expect("<ChatService>::ERROR::TWITCH LISTEN FAILED");
}

#[cfg(test)]
mod tests {
    use super::*;

    use strausslib::redisclient::mockredisclient::RedisClient;

    #[tokio::test]
    async fn chat_main() {
        let conf = ChatConfig::default();
        let mut _dut = ChatService {
            conf,
            redis: RedisClient::default(),
        };
    }
}
