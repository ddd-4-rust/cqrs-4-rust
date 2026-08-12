# Phase 2 — ESC 实施计划（6 个职责 + 共享引擎）

> 母计划: [`./2026-07-23-cqrs-4-rust-140-file-migration.md`](./2026-07-23-cqrs-4-rust-140-file-migration.md)
> 设计: [`../specs/2026-07-23-cqrs-4-rust-esc-design.md`](../specs/2026-07-23-cqrs-4-rust-esc-design.md)

---

## Task 2.1: 创建 `crates/esc` 骨架

- [ ] **Step 1: 创建 `crates/esc/Cargo.toml`**

```toml
[package]
name = "cqrs-4-rust-esc"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true

[dependencies]
cqrs-4-rust-core = { path = "../core", version = "0.1.0" }
thiserror = { workspace = true }
tokio = { workspace = true, features = ["sync"] }
async-trait = { workspace = true }
parking_lot = { workspace = true }
```

- [ ] **Step 2: `src/lib.rs` 模块声明**

```rust
pub mod jpa_event_dispatcher;
pub mod projection_service;
pub mod simple_jpa_event_dispatcher;

pub mod projection_admin;
pub mod event_decoder;
pub mod view_projector;
pub mod projection_stream_id;
pub mod managed_view;

pub mod memory;
```

- [ ] **Step 3: 验证骨架可编译**

```bash
cargo check -p cqrs-4-rust-esc --all-targets --all-features
```

- [ ] **Step 4: 提交**

```bash
git add crates/esc
git commit -m "feat(esc): skeleton crate with module declarations"
```

---

## Task 2.2: 实现 `JpaEventDispatcher` / `SimpleJpaEventDispatcher`

**Files:**
- `crates/esc/src/jpa_event_dispatcher.rs`
- `crates/esc/src/simple_jpa_event_dispatcher.rs`
- `crates/esc/tests/simple_jpa_event_dispatcher_test.rs`

- [ ] **Step 1: 写失败测试**

```rust
use cqrs_4_rust_esc::SimpleJpaEventDispatcher;
use cqrs_4_rust_core::{Event, JpaEventHandler};

struct Created(String);
impl Event for Created { fn event_type(&self) -> &'static str { "Created" } }

struct CountingHandler(usize);
impl JpaEventHandler<Created> for CountingHandler {
    fn event_type(&self) -> &'static str { "Created" }
    fn handle(&self, _: &Created) { /* mutate */ }
}

#[test]
fn single_handler_dispatches() { /* ... */ }

#[test]
fn multiple_handlers_for_same_event_run_in_registration_order() { /* ... */ }

#[test]
fn dispatching_unknown_event_type_returns_error() { /* ... */ }
```

- [ ] **Step 2: 实现 `JpaEventDispatcher` trait + 类型擦除**

```rust
pub trait JpaEventHandlerErased: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn handle_boxed(&self, event: &dyn std::any::Any) -> Result<(), DispatchError>;
}

pub trait JpaEventDispatcher: Send + Sync {
    fn register<E, H>(&mut self, handler: H)
    where E: Event + 'static, H: JpaEventHandler<E> + 'static;
    fn dispatch_event(&self, event_type: &str, event: &dyn Event) -> Result<(), DispatchError>;
}
```

- [ ] **Step 3: 实现 `SimpleJpaEventDispatcher`**

```rust
pub struct SimpleJpaEventDispatcher { /* HashMap<&'static str, Vec<Box<dyn JpaEventHandlerErased>>> */ }
```

- [ ] **Step 4: 运行测试，确认通过**

```bash
cargo test -p cqrs-4-rust-esc --test simple_jpa_event_dispatcher_test
```

- [ ] **Step 5: 提交**

```bash
git add crates/esc/src/jpa_event_dispatcher.rs crates/esc/src/simple_jpa_event_dispatcher.rs crates/esc/tests/simple_jpa_event_dispatcher_test.rs
git commit -m "feat(esc): JpaEventDispatcher + SimpleJpaEventDispatcher"
```

---

## Task 2.3: 实现 `ProjectionService`

**Files:**
- `crates/esc/src/projection_service.rs`

- [ ] **Step 1: 实现 trait + 内存实现骨架**

```rust
pub trait ProjectionService: Send + Sync {
    fn read_projection_position(&self, stream_id: &StreamId) -> Option<u64>;
    fn update_projection_position(&self, stream_id: &StreamId, next: u64) -> Result<(), ProjectionError>;
    fn reset_projection_position(&self, stream_id: &StreamId) -> Result<(), ProjectionError>;
}
```

- [ ] **Step 2: 提交**

```bash
git add crates/esc/src/projection_service.rs crates/esc/src/lib.rs
git commit -m "feat(esc): ProjectionService trait"
```

---

## Task 2.4: 实现共享引擎（Rust 新增，不占用 140 个迁移职责）

**Files:**
- `crates/esc/src/projection_admin.rs`
- `crates/esc/src/event_decoder.rs`
- `crates/esc/src/view_projector.rs`
- `crates/esc/src/managed_view.rs`
- `crates/esc/src/projection_stream_id.rs`
- `crates/esc/src/memory.rs`（内存实现）

- [ ] **Step 1: 实现 `ProjectionStreamId`**

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionStreamId(String);

impl ProjectionStreamId {
    pub fn new(name: impl Into<String>) -> Self { Self(name.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}
```

- [ ] **Step 2: 实现 `ProjectionAdmin` trait + `MemoryProjectionAdmin`**

- [ ] **Step 3: 实现 `EventDecoder` trait + `RegistryEventDecoder`**

- [ ] **Step 4: 实现 `ViewProjector::tick`**

```rust
pub struct TickOutcome { pub stream_id: ProjectionStreamId, pub advanced: u64, pub events: usize }

impl<ES, PA, PS, ED> ViewProjector<ES, PA, PS, ED>
where ES: EventStore + Send + Sync + 'static,
      PA: ProjectionAdmin + Send + Sync + 'static,
      PS: ProjectionService + Send + Sync + 'static,
      ED: EventDecoder + Send + Sync + 'static,
{
    pub async fn tick<V: View<E = ...>>(&self, view: &V) -> Result<TickOutcome, ProjectionError> {
        // 1. 投影位置
        let pos = self.projection_service.read_projection_position(&view.get_projection_stream_id()).unwrap_or(0);
        // 2. 读 chunk
        let slice = self.event_store.read_slice(...).await?;
        // 3. 解码 + 派发
        for raw in slice.events() { let event = self.event_decoder.decode(raw)?; view.handle_events(&[event]); }
        // 4. 更新位置
        self.projection_service.update_projection_position(&view.get_projection_stream_id(), slice.next_position())?;
        Ok(TickOutcome { /* ... */ })
    }
}
```

- [ ] **Step 5: 实现 `ManagedView`**

```rust
pub struct ManagedView<V> { pub view: V, pub lock: Arc<tokio::sync::Semaphore> }
```

- [ ] **Step 6: 写 `tick_advances_position_and_dispatches_events` 测试**

```rust
#[tokio::test]
async fn projection_tick_advances_position_and_dispatches_events() {
    // 构造内存 EventStore / ProjectionAdmin / ProjectionService
    // tick → view 收到事件且位置前进
    // 二次 tick → 不重复
}
```

- [ ] **Step 7: 运行测试**

```bash
cargo test -p cqrs-4-rust-esc --all-features
```

- [ ] **Step 8: 提交**

```bash
git add crates/esc/src/projection_admin.rs crates/esc/src/event_decoder.rs crates/esc/src/view_projector.rs crates/esc/src/managed_view.rs crates/esc/src/projection_stream_id.rs crates/esc/src/memory.rs crates/esc/src/lib.rs
git commit -m "feat(esc): shared projection engine (ProjectionAdmin/EventDecoder/ViewProjector)"
```

---

## Task 2.5: 终态校验

- [ ] **Step 1: 全量门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test -p cqrs-4-rust-esc --all-features
cargo clippy -p cqrs-4-rust-esc --all-targets --all-features -- -D warnings
./tools/check_migration_parity.sh
```

- [ ] **Step 2: 提交 Phase 2 完成标记**

```bash
git commit --allow-empty -m "chore(esc): Phase 2 migration complete"
```

---

## 完成标志

- 6 个迁移职责齐全（`jpa_event_dispatcher` / `simple_jpa_event_dispatcher` / `projection_service` + 3 tests）
- 共享引擎存在但**不占用 140 计数**
- `projection_tick_advances_position_and_dispatches_events` 通过
- `tools/check_migration_parity.sh` 通过