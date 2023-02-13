/*
redisclient.rs
Copyright (C) 2023
Squidpie
*/

//! RedisClient implementation for RedisWrapper
//! Wraps redis calls in traits

use async_trait::async_trait;
use futures_util::{pin_mut, StreamExt};
use mockall::mock;
use redis::aio;
use redis::aio::Connection;
use redis::Client;
use std::error::Error;

pub const STRAUSS_REDIS_ADDR: &str = "redis://redis:6379";

pub trait RedisClient: Publish + PubSub {}

#[async_trait(?Send)]
pub trait Publish {
    async fn publish(&mut self, ch: String, msg: String) -> Result<(), Box<dyn Error>>;
}

#[async_trait(?Send)]
pub trait PubSub {
    async fn pubsub(&mut self) -> String;
}

impl std::fmt::Debug for Box<dyn RedisClient> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RedisClient: Publish + PubSub {}")
    }
}

pub struct RedisWrapper {
    con: Connection,
    pubsub: aio::PubSub,
}

#[async_trait(?Send)]
impl Publish for RedisWrapper {
    async fn publish(&mut self, ch: String, msg: String) -> Result<(), Box<dyn Error>> {
        redis::cmd("PUBLISH")
            .arg(&[ch, msg])
            .query_async(&mut self.con)
            .await?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl PubSub for RedisWrapper {
    async fn pubsub(&mut self) -> String {
        let pubsub = self.pubsub.on_message();
        pin_mut!(pubsub);
        let msg = pubsub
            .next()
            .await
            .expect("<RedisWrapper>::ERROR::FAILED TO GET NEXT MESSAGE");
        msg.get_payload().unwrap()
    }
}

impl RedisClient for RedisWrapper {}

impl RedisWrapper {
    pub async fn new(ch: String) -> RedisWrapper {
        let client =
            Client::open(STRAUSS_REDIS_ADDR).expect("<RedisWrapper>::ERROR::FAILED TO OPEN REDIS");
        let mut pubsub = client
            .get_async_connection()
            .await
            .expect("<RedisWrapper>::ERROR::FAILED TO GET PUBSUB CON")
            .into_pubsub();
        pubsub
            .subscribe(ch)
            .await
            .expect("<RedisWrapper>::ERROR::FAILED TO SUB TO CHANNEL");
        let con = client
            .get_async_connection()
            .await
            .expect("<RedisWrapper>::ERROR::FAILED TO GET REDIS CON");
        let wrapper = RedisWrapper { con, pubsub };
        println!("<RedisWrapper>::INFO::Redis Connection Established");
        wrapper
    }
}

mock! {
    pub RedisWrapper{}
    #[async_trait(?Send)]
    impl Publish for RedisWrapper{
        async fn publish(&mut self, ch: String, msg: String) ->  Result<(), Box<dyn Error>>;
    }
    #[async_trait(?Send)]
    impl PubSub for RedisWrapper {
        async fn pubsub(&mut self) -> String;
    }
    impl RedisClient for RedisWrapper{}
}
