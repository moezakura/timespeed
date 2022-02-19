use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    speed_test_binary: String,
}

impl Config {
    pub fn new(file_path: String) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut s = String::new();
        let _ = file.read_to_string(&mut s)?;

        let deserialized_config: Config = serde_yaml::from_str(&s)?;
        Ok(deserialized_config)
    }
}
