//! Shared person-view contract from the Quarkus test model.

use crate::generated::PersonCreatedEvent;

/// Contract implemented by the person projection.
pub trait AbstractPersonsView {
    /// Stable view name.
    fn name(&self) -> &'static str;

    /// CRON expression used by the adapter.
    fn cron(&self) -> &'static str;

    /// Applies one person-created event.
    fn handle(&self, event: &PersonCreatedEvent) -> impl Future<Output = bool> + Send;
}
