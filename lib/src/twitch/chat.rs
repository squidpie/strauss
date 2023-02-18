/*
chat.rs
Copyright (C) 2023
Squidpie
*/

pub use twitch_irc::message::PrivmsgMessage as TwitchMsg;

pub const STRAUSS_CHAT_MSG_RX_REDIS_CH: &str = "#strauss-chat-msg-rx";
pub const STRAUSS_CHAT_MSG_TX_REDIS_CH: &str = "#strauss-chat-msg-tx";