//! Event store configuration for the Axum adapter.
//!
//! 1:1 translation of `org.fuin.cqrs4j.springboot.base.EventstoreConfig`.

use serde::{Deserialize, Serialize};

/// Spring Boot event-store properties represented as Axum application state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStoreConfig {
    /// Whether transport encryption is enabled.
    #[serde(default)]
    pub tls: bool,
    /// Event store host name.
    #[serde(default = "default_host")]
    pub host: String,
    /// Event store HTTP or HTTPS port.
    #[serde(default = "default_port")]
    pub port: u16,
    /// Optional event store user name.
    #[serde(default)]
    pub username: String,
    /// Optional event store password.
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
