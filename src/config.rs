use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::exit};

use crate::arg::Args;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub exact: Vec<String>,
    pub wildcard: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // pub log_level: Option<String>,
    pub log_path: Option<PathBuf>,
    // Listen port
    pub port: Option<usize>,
    pub list: List,
}

impl Config {
    pub fn build() -> Self {
        let args = Args::parse();

        // Read config file location from args.
        let config_path = if let Some(path) = args.config {
            path
        } else {
            PathBuf::from("./config.json")
        };
        let config = match fs::read_to_string(&config_path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!(
                    "Error: failed to read config file {:?}, {:?}",
                    config_path, err
                );
                exit(1);
            }
        };
        let mut config: Config = serde_json::from_str(&config).expect("Config file format error");

        // Initial default config.
        if config.port.is_none() {
            config.port = Some(3000)
        }
        // if config.log_level.is_none() {
        //     config.log_level = Some("info".to_owned());
        // }
        if config.log_path.is_none() {
            config.log_path = Some(PathBuf::from("/tmp/rua-list/log"))
        }

        config
    }
}