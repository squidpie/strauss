/*
redisclient.rs
Copyright (C) 2023
Squidpie
*/

use redis::{Client, Connection, PubSub};
use std::error::Error;

pub const STRAUSS_REDIS_ADDR: &str = "redis://redis:6379";

pub struct RedisClient {
    con: Connection,
}

impl Default for RedisClient {
    fn default() -> Self {
        let c = Client::open(STRAUSS_REDIS_ADDR)
            .expect("<RedisClient>::ERROR::FAILED TO OPEN REDIS");
        let rc = RedisClient {
            con: c
                .get_connection()
                .expect("<RedisClient>::ERROR::FAILED TO GET REDIS CON"),
        };
        println!("<RedisClient>::INFO::Redis Connection Established");
        rc
    }
}

impl RedisClient {
    pub fn publish(&mut self, ch: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        redis::cmd("PUBLISH")
            .arg(ch)
            .arg(msg)
            .query(&mut self.con)?;
        Ok(())
    }

    pub async fn pubsub(&mut self, ch: &str) -> Result<PubSub, Box<dyn Error>> {
        let mut pubsub = self.con.as_pubsub();
        pubsub.subscribe(ch)?;
        Ok(pubsub)
    }
}

pub mod mockredisclient {

    #[derive(Default)]
    pub struct RedisClient;

    impl RedisClient {
        pub fn publish(&mut self, ch: &str, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
          Ok(println!("::TEST OUTPUT:: [{ch}] {msg}"))
        }
    }
}
