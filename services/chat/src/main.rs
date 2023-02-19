/*
main.rs
Copyright (C) 2023
Squidpie
*/

//! Strauss Chat uService
//!
//! Transports Twitch chat messages to Redis STRAUSS_CHAT_MSG_RX_REDIS_CH as JSON Strings.
//!
//! Transports Redis STRAUSS_CHAT_MSG_TX_REDIS_CH messages to Twitch Chat

use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use tokio::task::LocalSet;

use strausslib::config::{FileReader, StraussConfigLoad};
use strausslib::redisclient::{RedisClient, RedisWrapper};
use strausslib::twitch::chat::STRAUSS_CHAT_MSG_RX_REDIS_CH;
use strausslib::twitch::chat::STRAUSS_CHAT_MSG_TX_REDIS_CH;
use strausslib::twitch::config::TwitchConfig;

mod chatclient;
use chatclient::{ChatClient, ChatWrapper};

#[derive(Debug)]
struct RedisListener {
    twitch: Box<dyn ChatClient>,
    redis: Box<dyn RedisClient>,
}

#[derive(Debug)]
struct TwitchListener {
    twitch: Box<dyn ChatClient>,
    redis: Box<dyn RedisClient>,
}

#[async_trait(?Send)]
trait Listen {
    async fn listen(&mut self);
}

#[async_trait(?Send)]
impl Listen for RedisListener {
    async fn listen(&mut self) {
        let val = self.redis.pubsub().await;
        println!("::DEBUG OUTPUT::RedisListener Processing Message");
        self.twitch.say(val).await;
    }
}

#[async_trait(?Send)]
impl Listen for TwitchListener {
    async fn listen(&mut self) {
        if let Some(json) = self.twitch.recv().await {
            println!("::DEBUG OUTPUT::TwitchListener Processing Message");
            self.redis
                .publish(
                    STRAUSS_CHAT_MSG_RX_REDIS_CH.to_owned(),
                    String::from(json.as_str()),
                )
                .await
                .expect("<TwitchListener>::ERROR::FAILED TO PUBSLISH REDIS MSG");
        }
    }
}

async fn run() -> Pin<Box<dyn Future<Output = ()>>> {
    let yml_reader = Box::new(FileReader {});
    let config = StraussConfigLoad::load(yml_reader);
    let config = Box::new(TwitchConfig::load(config));

    let twitch = Box::new(ChatWrapper::new(config.clone()));
    let redis = Box::new(RedisWrapper::new(STRAUSS_CHAT_MSG_TX_REDIS_CH.to_owned()).await);
    let mut redis_listener = RedisListener { twitch, redis };

    let twitch = Box::new(ChatWrapper::new(config.clone()));
    let redis = Box::new(RedisWrapper::new(STRAUSS_CHAT_MSG_RX_REDIS_CH.to_owned()).await);
    let mut twitch_listener = TwitchListener { twitch, redis };

    println!("<ChatService>::INFO::Listeners Created, Entering Run Loop");

    Box::pin(async {
        let t = tokio::task::spawn_local(async move {
            loop {
                twitch_listener.listen().await;
            }
        });

        let r = tokio::task::spawn_local(async move {
            loop {
                redis_listener.listen().await;
            }
        });

        t.await.expect("<ChatService>::ERROR::TWITCH THREAD FAILED");
        r.await.expect("<ChatService>::ERROR::REDIS THREAD FAILED");
    })
}

/// Main
///
#[tokio::main]
pub async fn main() {
    let local = LocalSet::new();
    local.run_until(run().await).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    use chatclient::MockChatWrapper;
    use strausslib::redisclient::MockRedisWrapper;

    #[tokio::test]
    async fn listen_twitch() {
        let mut twitch = MockChatWrapper::default();
        twitch
            .expect_recv()
            .once()
            .returning(|| Some(String::from("")));

        let mut redis = MockRedisWrapper::default();
        redis.expect_publish().once().returning(|_, _| Ok(()));

        let mut dut = TwitchListener {
            twitch: Box::new(twitch),
            redis: Box::new(redis),
        };
        dut.listen().await;
    }

    #[tokio::test]
    async fn listen_redis() {
        let mut twitch = MockChatWrapper::default();
        twitch.expect_say().once().returning(|_| {});

        let mut redis = MockRedisWrapper::default();
        redis
            .expect_pubsub()
            .once()
            .returning(|| String::from("test message"));

        let mut dut = RedisListener {
            twitch: Box::new(twitch),
            redis: Box::new(redis),
        };
        dut.listen().await;
    }
}
