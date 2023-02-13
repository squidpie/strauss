/*
chatclient.rs
Copyright (C) 2023
Squidpie
*/

//! ChatClient implementation for TwitchWrapper
//! Wraps twitch_irc calls in traits

use async_trait::async_trait;
use mockall::mock;
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use strausslib::twitch::config::Config;

#[derive(Debug)]
pub struct ChatWrapper {
    ch: String,
    client: Box<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    incoming_msg: Box<UnboundedReceiver<ServerMessage>>,
}

impl ChatWrapper {
    pub fn new(config: Box<dyn Config>) -> Self {
        let (login_name, oauth_token) = config.credentials();
        let client_config =
            ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(oauth_token)));
        let (incoming_msg, client): (
            UnboundedReceiver<ServerMessage>,
            TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        ) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);
        client
            .join(config.channel())
            .expect("<ChatWrapper>::ERROR::FAILED TO JOIN CHANNEL");
        println!(
            "<ChatWrapper>::INFO::Twitch Transport Client Joined Channel {0} | {client:?}",
            config.channel()
        );
        ChatWrapper {
            ch: config.channel(),
            client: Box::new(client),
            incoming_msg: Box::new(incoming_msg),
        }
    }
}

#[async_trait(?Send)]
pub trait Recv {
    async fn recv(&mut self) -> Option<String>;
}

#[async_trait(?Send)]
pub trait Say {
    async fn say(&self, msg: String);
}
pub trait ChatClient: Recv + Say {}

impl std::fmt::Debug for Box<dyn ChatClient> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ChatClient: Recv + Say {}")
    }
}

#[async_trait(?Send)]
impl Say for ChatWrapper {
    async fn say(&self, msg: String) {
        self.client
            .say(self.ch.to_owned(), msg)
            .await
            .expect("<ChatWrapper>::ERROR::TWITCH SAY FAILED");
    }
}

#[async_trait(?Send)]
impl Recv for ChatWrapper {
    async fn recv(&mut self) -> Option<String> {
        let message = self
            .incoming_msg
            .recv()
            .await
            .expect("<ChatWrapper>::ERROR::FAILED TO RECV MESSAGE");
        let json = match message {
            ServerMessage::Privmsg(priv_msg) => Some(serde_json::to_string(&priv_msg).unwrap()),
            _ => self.recv().await,
        };
        json
    }
}

impl ChatClient for ChatWrapper {}

mock! {
    pub ChatWrapper {}
    #[async_trait(?Send)]
    impl Say for ChatWrapper {
        async fn say(&self, msg: String);
    }
    #[async_trait(?Send)]
    impl Recv for ChatWrapper {
        async fn recv(&mut self) -> Option<String>;
    }
    impl ChatClient for ChatWrapper {}
}
