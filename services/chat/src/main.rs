/*
main.rs
Copyright (C) 2023
Squidpie
*/

use strausslib::redisclient::RedisClient;
struct ChatService<T> {
    redis: T,
}

fn main() {
    let mut chat = ChatService {
        redis: RedisClient::new()
    };
    chat.redis
        .publish("meow", "this is a meow")
        .ok()
        .expect("<chat>::ERROR::FAILED TO PUBLISH");
}

#[cfg(test)]
mod tests {
    use super::*;

    use strausslib::redisclient::mockredisclient::RedisClient;

    #[test]
    fn chat_main() {
        let redis = RedisClient::new();
        let mut dut = ChatService { redis: redis };
        dut.redis
            .publish("test", "0xdeadbeef")
            .ok()
            .expect("::TEST OUTPUT::PUBLISH FAILED")
    }
}
