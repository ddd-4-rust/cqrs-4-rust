//! Event store configuration.
//!
//! Shared config type (identical to actix version).

use serde::{Deserialize, Serialize};

/// Configuration for connecting to an event store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStoreConfig {
    #[serde(default)]
    pub tls: bool,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    2113
}

impl Default for EventStoreConfig {
    fn default() -> Self {
        Self {
            tls: false,
            host: default_host(),
            port: default_port(),
            username: String::new(),
            password: String::new(),
        }
    }
}
