/*
redisclient.rs
Copyright (C) 2023
Squidpie
*/

pub use redis::{Client, Connection};
use std::error::Error;

pub struct RedisClient {
    con: Connection,
}

impl RedisClient {
    pub fn new() -> RedisClient {
        let c = Client::open("redis://0.0.0.0:6379")
            .ok()
            .expect("<RedisClient>::ERROR::FAILED TO OPEN REDIS");
        RedisClient {
            con: c
                .get_connection()
                .ok()
                .expect("<RedisClient>::ERROR::FAILED TO GET REDIS CON"),
        }
    }

    pub fn publish(&mut self, ch: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        let _: () = redis::cmd("PUBLISH")
            .arg(ch)
            .arg(msg)
            .query(&mut self.con)?;
        Ok(())
    }
}

pub mod mockredisclient {
    pub struct RedisClient;

    impl RedisClient {
        pub fn new() -> Self {
            Self
        }
        pub fn publish(&mut self, _ch: &str, _msg: &str) -> Result<(), Box<dyn std::error::Error>> {
          Ok(println!("::TEST OUTPUT:: [{_ch}] {_msg}"))
        }
    }
}
