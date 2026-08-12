# Phase 4 — 框架适配实施计划（20 个职责）

> 母计划: [`./2026-07-23-cqrs-4-rust-140-file-migration.md`](./2026-07-23-cqrs-4-rust-140-file-migration.md)
> 设计: [`../specs/2026-07-23-cqrs-4-rust-framework-adapter-design.md`](../specs/2026-07-23-cqrs-4-rust-framework-adapter-design.md)

---

## Task 4.1: 创建 axum / actix 适配骨架

- [ ] **Step 1: 注册 crate 到 workspace**

- [ ] **Step 2: 每个 crate 的 `Cargo.toml`**

```toml
# crates/adapter/axum/Cargo.toml
[package]
name = "cqrs-4-rust-axum"
...
[dependencies]
cqrs-4-rust-core = { path = "../../core", version = "0.1.0" }
cqrs-4-rust-esc = { path = "../../esc", version = "0.1.0" }
axum = { workspace = true }
tokio = { workspace = true, features = ["full"] }
tokio-cron-scheduler = { workspace = true }
async-trait = { workspace = true }
thiserror = { workspace = true }

# crates/adapter/actix/Cargo.toml
[package]
name = "cqrs-4-rust-actix"
...
[dependencies]
cqrs-4-rust-core = { path = "../../core", version = "0.1.0" }
cqrs-4-rust-esc = { path = "../../esc", version = "0.1.0" }
actix-web = { workspace = true }
actix-rt = { workspace = true }
tokio-cron-scheduler = { workspace = true }
async-trait = { workspace = true }
thiserror = { workspace = true }
```

- [ ] **Step 3: `src/lib.rs` 声明 4 个模块**

- [ ] **Step 4: 验证骨架可编译**

```bash
cargo check -p cqrs-4-rust-axum -p cqrs-4-rust-actix --all-targets --all-features
```

- [ ] **Step 5: 提交**

```bash
git add crates/adapter
git commit -m "feat(adapter): skeleton crates (axum/actix)"
```

---

## Task 4.2: 实现 `EventStoreConfig`（两套等价）

**Files:**
- `crates/adapter/axum/src/event_store_config.rs`
- `crates/adapter/actix/src/event_store_config.rs`
- `crates/adapter/axum/tests/event_store_config_test.rs`
- `crates/adapter/actix/tests/event_store_config_test.rs`

- [ ] **Step 1: 写失败测试**

```rust
#[test]
fn defaults_when_env_missing() {
    let cfg = EventStoreConfig::from_env().unwrap();
    assert_eq!(cfg.protocol, Protocol::Http);
    assert_eq!(cfg.host, "127.0.0.1");
    assert_eq!(cfg.port, 2113);
    assert!(!cfg.tls);
}

#[test]
fn port_zero_is_rejected() {
    std::env::set_var("EVENT_STORE_PORT", "0");
    assert!(EventStoreConfig::from_env().is_err());
}

#[test]
fn tls_with_http_is_rejected() {
    std::env::set_var("EVENT_STORE_TLS", "true");
    std::env::set_var("EVENT_STORE_PROTOCOL", "http");
    assert!(EventStoreConfig::from_env().is_err());
}
```

- [ ] **Step 2: 实现 `EventStoreConfig`**

```rust
pub struct EventStoreConfig {
    pub protocol: Protocol,
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl EventStoreConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let protocol = Protocol::from_env()?;
        let host = std::env::var("EVENT_STORE_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("EVENT_STORE_PORT").ok().map_or(Ok(2113), |s| s.parse::<u16>()).map_err(|_| ConfigError::InvalidPort)?;
        let tls = std::env::var("EVENT_STORE_TLS").map(|v| v == "true").unwrap_or(false);
        if tls && matches!(protocol, Protocol::Http) { return Err(ConfigError::TlsWithHttp); }
        if host.is_empty() { return Err(ConfigError::EmptyHost); }
        Ok(Self { protocol, host, port, tls, username: None, password: None })
    }
}
```

- [ ] **Step 3: 提交**

```bash
git add crates/adapter/{axum,actix}
git commit -m "feat(adapter): EventStoreConfig with env parsing + validation"
```

---

## Task 4.3: 实现 `QueryProjectionPosition` / `QueryProjectionService`（axum）/ `QueryProjectionPositionRepository`（actix）

**Files:**
- `crates/adapter/axum/src/query_projection_position.rs`
- `crates/adapter/axum/src/query_projection_service.rs`
- `crates/adapter/actix/src/query_projection_position.rs`
- `crates/adapter/actix/src/query_projection_position_repository.rs`

- [ ] **Step 1: axum 侧实现**

`QueryProjectionPosition` 对应 Java `@Entity`（在 axum 侧使用 `sqlx`），`QueryProjectionService` 走 `ProjectionService` trait。

- [ ] **Step 2: actix 侧实现**

`QueryProjectionPosition` 同上；`QueryProjectionPositionRepository` 是 `ProjectionService` 的具体实现（Quarkus 端 Java 把这两者合并，Rust 端按文件 1:1 拆分）。

- [ ] **Step 3: 单元测试**

- [ ] **Step 4: 提交**

```bash
git add crates/adapter/{axum,actix}
git commit -m "feat(adapter): QueryProjectionPosition + QueryProjectionService"
```

---

## Task 4.4: 实现 `AxumJpaViewManager` / `ActixJpaViewManager`

**Files:**
- `crates/adapter/axum/src/axum_jpa_view_manager.rs`
- `crates/adapter/actix/src/actix_jpa_view_manager.rs`
- `crates/adapter/axum/tests/axum_jpa_view_manager_test.rs`
- `crates/adapter/actix/tests/actix_jpa_view_manager_test.rs`

- [ ] **Step 1: 写失败测试**

```rust
#[tokio::test]
async fn start_stops_with_cron_jobs() {
    let mgr = AxumJpaViewManager::new(/* ... */);
    mgr.start().await.unwrap();
    // 0.1s 内不应有事件
    tokio::time::sleep(Duration::from_millis(100)).await;
    mgr.stop().await.unwrap();
}

#[tokio::test]
async fn tick_all_advances_position_with_memory_eventstore() {
    // 写事件 → tick_all → view 收到且位置前进
    // 二次 tick_all → 不重复
}
```

- [ ] **Step 2: 实现 `AxumJpaViewManager`**

```rust
pub struct AxumJpaViewManager<ES, PA, PS, ED> {
    pub event_store: Arc<ES>,
    pub projection_admin: Arc<PA>,
    pub projection_service: Arc<PS>,
    pub event_decoder: Arc<ED>,
    pub views: Vec<Arc<dyn View<E = Box<dyn Event>>>>,
    pub scheduler: JobScheduler,
    pub locks: HashMap<String, Arc<tokio::sync::Semaphore>>,
    pub projector: Arc<ViewProjector<ES, PA, PS, ED>>,
}
```

- [ ] **Step 3: 实现 `start() / stop() / tick_all()`**

- [ ] **Step 4: 实现 `ActixJpaViewManager`（镜像）**

- [ ] **Step 5: 提交**

```bash
git add crates/adapter/{axum,actix}
git commit -m "feat(adapter): Axum/Actix JpaViewManager with CRON + tick_all"
```

---

## Task 4.5: 实现 `architecture_test.rs` / `base_test.rs`（两套）

- [ ] **Step 1: 写架构守门测试**

- [ ] **Step 2: 写 base_test 共享 fixture**

- [ ] **Step 3: 提交**

```bash
git add crates/adapter/{axum,actix}/tests
git commit -m "test(adapter): architecture_test + base_test for axum/actix"
```

---

## Task 4.6: 终态校验

- [ ] **Step 1: 全量门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test -p cqrs-4-rust-axum --all-features
cargo test -p cqrs-4-rust-actix --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
./tools/check_migration_parity.sh
```

- [ ] **Step 2: 内存投影闭环**

```bash
cargo test -p cqrs-4-rust-axum --all-features --test axum_jpa_view_manager_test
cargo test -p cqrs-4-rust-actix --all-features --test actix_jpa_view_manager_test
```

- [ ] **Step 3: 提交 Phase 4 完成标记**

```bash
git commit --allow-empty -m "chore(adapter): Phase 4 migration complete"
```

---

## 完成标志

- 10 + 10 = 20 个迁移职责齐全
- 内存 EventStore 投影闭环通过
- `tools/check_migration_parity.sh` 通过