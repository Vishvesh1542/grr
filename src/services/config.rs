use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bar_height: i32,
    pub time_format: String,
    pub margin: i32,
    pub spacing: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bar_height: 30,
            time_format: "%b %e %l:%M %p".to_string(),
            margin: 5,
            spacing: 15,
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Config not initialized!")
}

pub fn init_config() {
    let cfg: Config = confy::load("grr", "config").expect("Failed to load or create config");

    CONFIG.set(cfg).expect("Config already initialized");
}
