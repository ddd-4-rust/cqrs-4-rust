//! Public ESC API architecture checks.

use cqrs_4_rust_esc::{JpaEventDispatcher, ProjectionService, SimpleJpaEventDispatcher};

#[test]
fn public_facade_exposes_esc_contracts() {
    fn assert_dispatcher<T: JpaEventDispatcher>() {}
    fn assert_projection_service<T: ProjectionService>() {}
    let _ = assert_dispatcher::<SimpleJpaEventDispatcher>;
    let _ = assert_projection_service::<NeverProjectionService>;
}

struct NeverProjectionService;

#[async_trait::async_trait]
impl ProjectionService for NeverProjectionService {
    async fn reset_projection_position(
        &self,
        _stream_id: &str,
    ) -> Result<(), cqrs_4_rust_esc::ProjectionError> {
        Ok(())
    }

    async fn read_projection_position(
        &self,
        _stream_id: &str,
    ) -> Result<Option<cqrs_4_rust_esc::ProjectionPosition>, cqrs_4_rust_esc::ProjectionError> {
        Ok(None)
    }

    async fn update_projection_position(
        &self,
        _stream_id: &str,
        _position: &cqrs_4_rust_esc::ProjectionPosition,
    ) -> Result<(), cqrs_4_rust_esc::ProjectionError> {
        Ok(())
    }
}
