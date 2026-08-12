# cqrs-4-rust esc 设计

- 日期: 2026-07-23
- 范围: `crates/esc/` (6 个迁移职责 + 共享引擎)
- 状态: dispatcher 与内存投影 tick 已验收；KurrentDB 真投影与 MariaDB 持久化仍 defer

## 1. 背景

Java 版 `cqrs-4-java/esc` 提供 JPA 友好的事件分发：`JpaEventDispatcher` 接口、`SimpleJpaEventDispatcher` 默认实现、`ProjectionService` 抽象。每个事件类型可绑定多个 handler；分发失败时错误向上传播。

迁移必须保留"同一事件类型多 handler"的语义，并扩展出可复用的共享投影引擎（`ProjectionAdmin` / `EventDecoder` / `ViewProjector`），让 `adapter/axum` 与 `adapter/actix` 共享同一套投影逻辑。

## 2. 目标

1. `register` 真实保存类型擦除后的 handler。
2. `dispatch_event(s)` 按 `EventType` 路由并传播 handler 错误。
3. 一个事件类型可绑定多个 handler，handler 顺序与注册顺序一致。
4. 提供共享投影引擎，承载投影位置、EventStore 读流、错误恢复策略。

## 3. 非目标

不绑定具体 EventStore 实现（共享 `EventStore` trait），不绑定具体持久化（共享 `ProjectionPositionStore` trait）。

## 4. 模块布局

```
crates/esc/
├── src/
│   ├── lib.rs
│   ├── jpa_event_dispatcher.rs
│   ├── projection_service.rs
│   ├── simple_jpa_event_dispatcher.rs
│   ├── projection_admin.rs                # 共享引擎（trait + MemoryProjectionAdmin）
│   ├── event_decoder.rs                   # 共享引擎（trait + RegistryEventDecoder）
│   ├── view_projector.rs                  # 共享引擎（核心：read-stream + dispatch + tick）
│   ├── projection_stream_id.rs
│   └── managed_view.rs                    # 带 Semaphore 锁的视图封装
└── tests/
    ├── architecture_test.rs
    ├── base_test.rs
    └── simple_jpa_event_dispatcher_test.rs
```

## 5. 关键抽象

### 5.1 JpaEventDispatcher

```rust
pub trait JpaEventDispatcher {
    fn register<E: Event, H: JpaEventHandler<E>>(&mut self, handler: H)
    where H: 'static, E: 'static;

    fn dispatch_event(&self, event_type: &str, event: &dyn Event)
        -> Result<(), DispatchError>;

    fn dispatch_events(&self, events: &[&dyn Event])
        -> Result<(), DispatchError>;
}
```

类型擦除通过 `Box<dyn AnyHandler<E>>` + `EventType` 字符串映射实现。

### 5.2 ProjectionService

```rust
pub trait ProjectionService {
    fn read_projection_position(&self, stream_id: &StreamId) -> Option<u64>;
    fn update_projection_position(&self, stream_id: &StreamId, next: u64)
        -> Result<(), ProjectionError>;
    fn reset_projection_position(&self, stream_id: &StreamId)
        -> Result<(), ProjectionError>;
}
```

### 5.3 共享投影引擎（Rust 新增，不对应 Java 文件）

```rust
pub trait ProjectionAdmin: Send + Sync {
    fn projection_exists(&self, stream_id: &ProjectionStreamId) -> bool;
    fn create_projection(
        &self,
        stream_id: &ProjectionStreamId,
        emit_enabled: bool,
        type_names: &[TypeName],
    ) -> Result<(), ProjectionError>;
}

pub trait EventDecoder: Send + Sync {
    fn decode(&self, raw: &[u8]) -> Result<Box<dyn Event>, DecodeError>;
}

pub struct ViewProjector<ES, PA, PS, ED> {
    event_store: Arc<ES>,
    projection_admin: Arc<PA>,
    projection_service: Arc<PS>,
    event_decoder: Arc<ED>,
    chunk_size: usize,
}

impl<ES: EventStore, PA: ProjectionAdmin, PS: ProjectionService, ED: EventDecoder>
    ViewProjector<ES, PA, PS, ED>
{
    pub async fn tick<V: View>(&self, view: &V) -> Result<TickOutcome, ProjectionError>;
}
```

`MemoryProjectionAdmin` 与 `RegistryEventDecoder` 提供本地内存实现，作为测试与开发回路。

## 6. 错误语义

| Java | Rust | 传播路径 |
|---|---|---|
| `RuntimeException` 抛出 | `DispatchError::Handler(...)` | dispatch → caller |
| 数据库失败 | `ProjectionError::Store(...)` | projection service → caller |
| 投影位置冲突 | `ProjectionError::Conflict` | 同上 |
| 投影流已存在 | `ProjectionError::AlreadyExists` | projection admin → caller |
| 事件解码失败 | `DecodeError` | decoder → caller |

错误结构必须实现 `std::error::Error` + `source()`。

## 7. 测试覆盖

| 测试 | 覆盖 |
|---|---|
| `simple_jpa_event_dispatcher_test.rs` | 单 handler / 多 handler / 错误传播 / 空集合 / 未知事件 / 多 dispatch 顺序 |
| `architecture_test.rs` | trait 实现合规 |
| `base_test.rs` | 测试 fixture |

## 8. 完成定义

- 6 个迁移职责齐全 + 共享引擎存在但**不占用 140 个迁移职责**（仅 Java 文件映射项计 6）
- `cargo test -p cqrs-4-rust-esc --all-features` 通过
- 内存投影闭环测试通过：写事件 → tick → view 收到事件且位置前进 → 二次 tick 不重复

## 9. 与 Java 的差异

| 差异 | 解释 |
|---|---|
| 新增 `ProjectionAdmin` / `EventDecoder` / `ViewProjector` | Java 把这部分逻辑散布在 Quarkus / Spring Boot 适配层；Rust 抽取共享避免重复 |
| `ProjectionPositionStore` 抽象 | Java 直接走 JPA；Rust 通过 trait 抽象，方便测试 |
| `tick_all` 入口 | 给测试 / 手动推进使用，不在生产路径 |