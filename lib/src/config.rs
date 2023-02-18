/*
config.rs
Copyright (C) 2023
Squidpie
*/

//! Strauss Config Reader
//! Wrapper for File access. Parse yaml into HashMap
//!
//! Expected yaml format:
//! <service>:
//!     <option>:<value>

use std::{fs::File, collections::HashMap};
use serde::{Serialize, Deserialize};
use mockall::automock;

#[automock]
pub trait SerialRead<D: 'static + for<'de> Deserialize<'de>> {
    fn read(&self, path: String) -> D;
}


#[derive(Debug, Clone)]
pub struct FileReader;

impl<D:'static + for<'de> Deserialize<'de>> SerialRead<D> for FileReader {
    fn read(&self, path: String) -> D {
        let f = File::open(path).expect("<FileReader>::ERROR::FAILED TO OPEN FILE");
        serde_yaml::from_reader(f).expect("<FileReader>::ERROR::YAML SERIALIZE FAILED")
    }
}

pub trait ServiceConfigEntry {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StraussConfig {
    pub services: HashMap<String, HashMap<String, String>>
}

pub trait StraussConfigLoad {
    fn load(yml_reader: Box<dyn SerialRead<StraussConfig>>) -> Self;
}

#[automock]
impl StraussConfigLoad for StraussConfig {
    fn load(yml_reader: Box<dyn SerialRead<StraussConfig>>) -> Self {
        yml_reader.read(String::from("strauss.yml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::MockSerialRead;
    use std::collections::HashMap;

    #[test]
    fn load_config() {
        let mut yml_reader = MockSerialRead::<StraussConfig>::default();
        let mut services: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut chat_entry: HashMap<String, String> = HashMap::new();
        chat_entry.insert(String::from("channel"), String::from("test_channel"));
        services.insert(String::from("chat"), chat_entry);
        let config = StraussConfig{services};
        yml_reader
            .expect_read()
            .once()
            .returning(move |_| config.clone());

        let dut: StraussConfig = StraussConfigLoad::load(Box::new(yml_reader));
        assert_eq!(dut.services["chat"]["channel"], "test_channel");
    }
}