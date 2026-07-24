//! Utility functions for CQRS operations.

use adler2::Adler32;
use ddd_4_rust_core::EventType;

/// Utility functions for CQRS operations.
pub struct CqrsUtils;

impl CqrsUtils {
    /// Calculates the Java-compatible Adler-32 checksum in the supplied order.
    ///
    /// # Panics
    ///
    /// Panics when `event_types` is empty, matching the Java contract violation.
    pub fn calculate_adler32_checksum(event_types: &[EventType]) -> u32 {
        assert!(!event_types.is_empty(), "event_types cannot be empty");

        let mut checksum = Adler32::new();
        for event_type in event_types {
            checksum.write_slice(event_type.as_str().as_bytes());
        }
        checksum.checksum()
    }
}
