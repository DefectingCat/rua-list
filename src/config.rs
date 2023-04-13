use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    exact: Vec<String>,
    wildcard: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_info: Option<String>,
    pub list: List,
}

impl Config {
    pub fn build() -> Self {
        let config_path = PathBuf::from("./config.json");
        let config = fs::read_to_string(config_path).expect("Failed to read config file");
        let config: Config = serde_json::from_str(&config).expect("Config file format error");

        config
    }
}