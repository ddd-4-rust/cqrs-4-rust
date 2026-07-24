//! Shared container specifications for CQRS integration tests.

use std::collections::BTreeMap;
use std::time::Duration;

/// Declarative `EventStoreDB` container configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventStoreContainerSpec {
    /// Fully qualified container image.
    pub image: String,
    /// HTTP/gRPC port exposed by the image.
    pub exposed_port: u16,
    /// Environment passed to the container.
    pub environment: BTreeMap<String, String>,
    /// HTTP path used by readiness checks.
    pub readiness_path: String,
    /// Maximum readiness response duration.
    pub readiness_timeout: Duration,
}

/// Declarative `MariaDB` container configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MariaDbContainerSpec {
    /// Fully qualified container image.
    pub image: String,
    /// Database created for integration tests.
    pub database: String,
    /// Database user.
    pub username: String,
    /// Database password.
    pub password: String,
}

/// Builds the shared container contracts used by framework integration tests.
#[derive(Debug, Clone, Copy, Default)]
pub struct TestHelper;

impl TestHelper {
    /// Creates the Java-compatible in-memory `EventStoreDB` container specification.
    pub fn event_store(version: &str) -> EventStoreContainerSpec {
        EventStoreContainerSpec {
            image: format!("eventstore/eventstore:{version}"),
            exposed_port: 2113,
            environment: BTreeMap::from([
                ("EVENTSTORE_INSECURE".to_owned(), "true".to_owned()),
                (
                    "EVENTSTORE_LOG".to_owned(),
                    "/tmp/log-eventstore".to_owned(),
                ),
                ("EVENTSTORE_MEM_DB".to_owned(), "TRUE".to_owned()),
                ("EVENTSTORE_RUN_PROJECTIONS".to_owned(), "All".to_owned()),
            ]),
            readiness_path: "/web/index.html#/".to_owned(),
            readiness_timeout: Duration::from_secs(20),
        }
    }

    /// Creates the Java-compatible `MariaDB` container specification.
    pub fn maria_db(version: &str) -> MariaDbContainerSpec {
        MariaDbContainerSpec {
            image: format!("mariadb:{version}"),
            database: "testdb".to_owned(),
            username: "mary".to_owned(),
            password: "abc".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TestHelper;

    #[test]
    fn preserves_event_store_defaults() {
        let spec = TestHelper::event_store("24.10");
        assert_eq!(spec.image, "eventstore/eventstore:24.10");
        assert_eq!(spec.exposed_port, 2113);
        assert_eq!(spec.environment["EVENTSTORE_MEM_DB"], "TRUE");
    }

    #[test]
    fn preserves_maria_db_defaults() {
        let spec = TestHelper::maria_db("11");
        assert_eq!(spec.image, "mariadb:11");
        assert_eq!(spec.database, "testdb");
        assert_eq!(spec.username, "mary");
        assert_eq!(spec.password, "abc");
    }
}
