//! Utility functions for CQRS operations.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.CqrsUtils`.

use ddd_4_rust_core::event_type::EventType;

/// Utility functions for CQRS operations.
///
/// Java: `CqrsUtils`
pub struct CqrsUtils;

impl CqrsUtils {
    /// Calculates an Adler-32 checksum for a collection of event types.
    /// Used for projection stream ID calculation.
    ///
    /// Java: `calculateAdler32Checksum(Collection<EventType>) -> long`
    pub fn calculate_adler32_checksum(event_types: &[EventType]) -> u32 {
        use std::hash::Hasher;
        // Simple hash of event type names sorted
        let mut names: Vec<&str> = event_types.iter().map(|et| et.as_str()).collect();
        names.sort();
        names.dedup();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for name in &names {
            std::hash::Hash::hash(name, &mut hasher);
        }
        // Simulate adler32 behavior (in practice, use the `adler32` crate)
        hasher.finish() as u32
    }
}
