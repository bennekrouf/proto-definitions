use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub minioc_domain: String,
    pub minioc_port: u16,
    // other fields...
}

impl Config {
    pub fn from_yaml(path: &Path) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read YAML file");
        serde_yaml::from_str(&content).expect("Failed to parse YAML")
    }
}
