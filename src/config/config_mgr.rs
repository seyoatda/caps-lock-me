use std::fs::File;

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::VirtualKeySet;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub key_mappings: Vec<KeyMapping>,
}

#[derive(Deserialize, Serialize)]
pub struct KeyMapping {
    pub origin: VirtualKeySet,
    pub mapping: VirtualKeySet,
}

pub fn read_config(path: &Path) -> Config {
    let mut file = File::open(path).expect("failed to open file from path");
    let mut config: String = "".to_string();
    file.read_to_string(&mut config).expect("failed to read string from file");
    let config: Config = toml::from_str(config.as_str()).expect("failed to deserialize toml config");
    return config;
}
