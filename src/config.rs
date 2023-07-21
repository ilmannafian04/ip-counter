use std::process::exit;

use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AppConfig {
    #[serde(default = "d_host")]
    pub host: String,
    #[serde(default = "d_port")]
    pub port: u16,
}

fn d_host() -> String {
    "127.0.0.1".to_owned()
}
fn d_port() -> u16 {
    8080
}

impl AppConfig {
    pub fn new() -> Self {
        match envy::from_env::<AppConfig>() {
            Ok(app_config) => app_config,
            Err(err) => {
                error!("failed to initialize application: {err}");
                exit(1)
            }
        }
    }
}
