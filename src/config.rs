use std::env;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listenHost: String,
    pub listenPort: u16,
    pub metricBlacklist: Vec<String>,
}

pub fn parse_config() -> Config {
    let mut current_path = env::current_exe().expect("Unable to get exe directory");
    current_path.pop();

    let config_path = format!(
        "{}{}",
        current_path.display(),
        "/../../config/reference.json"
    );
    println!("loading configuration file {}", config_path);

    let contents = fs::read_to_string(config_path).expect("Unable to load configuration file");

    println!("{}", contents);

    let config: Config =
        serde_json::from_str(&contents).expect("Unable to decode configuration file");

    println!("{:?}", config.metricBlacklist);

    config
}