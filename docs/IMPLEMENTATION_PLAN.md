# cqrs-4-rust 实施计划

> 参照 ddd-4-rust/docs/IMPLEMENTATION_PLAN.md 同规格

## 当前状态

cqrs-4-rust 6 个 crate 编译通过，核心 trait 已 1:1 翻译：

| crate | 状态 |
|---|---|
| `cqrs-4-rust-core` | ✅ 核心 trait 完成（Command, CommandExecutor, Result, View, JpaEventHandler） |
| `cqrs-4-rust-serde` | ✅ AbstractCommand, AbstractAggregateCommand, SimpleResult, DataResult |
| `cqrs-4-rust-esc` | ✅ EventDispatcher, ProjectionService |
| `cqrs-4-rust-actix` | ⚠️ 骨架（ViewManager + EventStoreConfig） |
| `cqrs-4-rust-axum` | ⚠️ 骨架 |
| `cqrs-4-rust-test` | ⚠️ 仅占位 |

## Phase 2 补全计划

### 2.1 cqrs-4-rust-core 补全

- `to_result_capable.rs`：`ToResultCapable` trait（`fn to_result(&self) -> CqrsResult<T>`）

### 2.2 cqrs-4-rust-serde 补全

- `result_serde.rs`：`DataResult` 自定义 Serializer/Deserializer

### 2.3 cqrs-4-rust-actix/axum 补全

- `projection_position.rs`：`QryProjectionPosition` sqlx model + migration
- `view_manager.rs` 真实现：tokio-cron-scheduler CRON 调度 + EventStore 轮询

### 2.4 ddd-4-rust-codegen-processor 增强

- `#[derive(DddEvent)]` 派生宏：自动生成 Event + DomainEvent trait impl + serde
- `#[derive(EntityId)]` 派生宏：自动生成 EntityId trait impl

### 2.5 inventory-based EntityIdRegistry

- 替代 Java 的 Jandex 运行时扫描
- 使用 `inventory::submit!` 在编译期注册 EntityId 类型

## 依赖

```toml
# cqrs-4-rust-actix
actix-web = "4"
tokio-cron-scheduler = "0.13"
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
```
