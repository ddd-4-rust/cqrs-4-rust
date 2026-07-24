//! Event store configuration for the Actix adapter.
//!
//! 1:1 translation of `org.fuin.cqrs4j.quarkus.base.EventstoreConfig`.

use serde::{Deserialize, Serialize};

/// Configuration for connecting to an event store.
///
/// Java: Quarkus `EventstoreConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStoreConfig {
    /// Whether to use TLS.
    #[serde(default)]
    pub tls: bool,
    /// Event store host.
    #[serde(default = "default_host")]
    pub host: String,
    /// Event store HTTP port.
    #[serde(default = "default_port")]
    pub port: u16,
    /// Event store username.
    #[serde(default)]
    pub username: String,
    /// Event store password.
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
