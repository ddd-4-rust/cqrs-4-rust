# cqrs-4-rust 框架适配层设计

- 日期: 2026-07-23
- 范围: `crates/adapter/{axum,actix}/` (10 + 10 = 20 个迁移职责)
- 状态: 内存 EventStore 投影闭环已验收；KurrentDB 真投影 / MariaDB 持久化 defer

## 1. 背景

Java 版提供两套适配：`springboot`（Spring Boot）与 `quarkus`（Quarkus），分别承载相同的领域但绑定不同运行时。迁移约定 **Spring Boot ↔ Axum**、**Quarkus ↔ Actix**，这是因为 Rust 生态中：

- Axum 的中间件模型与 Spring WebFlux / Spring MVC 的声明式路由最接近；
- Actix 的 actor / handler 模型与 Quarkus 的 CDI `@ObservesAsync` 风格最接近。

## 2. 目标

1. 完成投影位置持久化、CRON 调度、并发互斥、EventStore 分块读取、独立事务、成功后更新位置和优雅停止。
2. `event_store_config` 保持 TLS、host、port 默认值及范围校验。
3. 注入 `EventStore` + `ProjectionAdmin` + `ProjectionService` + `EventDecoder`；`start()` 注册 CRON，`tick_all()` 供测试 / 手动推进，`stop()` 取消任务并 shutdown。
4. 共享引擎（`ViewProjector` / `ManagedView`）由 `esc` 提供；adapter 只承担"如何把引擎接到运行时"。

## 3. 非目标

- 不实现运行时内的 DI 容器（用 `trait` + 显式 wire）。
- 不实现 HTTP server 全栈（只提供 `PersonResource` / `PersonController` 等价路由）。
- 不替换 `tokio-cron-scheduler` 为自定义调度器。

## 4. 模块布局

```
crates/adapter/
├── axum/                            # 源 springboot (10)
│   ├── src/
│   │   ├── event_store_config.rs
│   │   ├── query_projection_position.rs
│   │   ├── query_projection_service.rs
│   │   ├── axum_jpa_view_manager.rs
│   │   └── lib.rs
│   └── tests/  (6)
│       ├── architecture_test.rs
│       ├── base_test.rs
│       ├── event_store_config_test.rs
│       ├── query_projection_position_test.rs
│       ├── query_projection_service_test.rs
│       └── axum_jpa_view_manager_test.rs
└── actix/                           # 源 quarkus (10)
    ├── src/
    │   ├── event_store_config.rs
    │   ├── query_projection_position.rs
    │   ├── query_projection_position_repository.rs   # 源 QuarkusJpaViewManager 拆分
    │   ├── actix_jpa_view_manager.rs
    │   └── lib.rs
    └── tests/  (6)
```

> Quarkus 在 Java 侧把 `ProjectionService` 合并进 `QuarkusJpaViewManager`；Rust 侧拆为独立文件 `query_projection_position_repository.rs`，保持一一对应（10 文件一致）。

## 5. 关键抽象

### 5.1 EventStoreConfig

```rust
pub struct EventStoreConfig {
    pub protocol: Protocol,         // default "http"
    pub host: String,               // default "127.0.0.1"
    pub port: u16,                  // default 2113
    pub tls: bool,                  // default false
    pub username: Option<String>,
    pub password: Option<String>,
}

impl EventStoreConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // host 非空、port 1..=65535、TLS 与 protocol 一致
    }
}
```

校验规则与 Java 一致：`host.is_empty() → error`、`port == 0 → error`、`tls && protocol == "http" → error` 等。

### 5.2 ViewManager（公共 trait，定义在 adapter crate）

```rust
#[async_trait]
pub trait ViewManager: Send + Sync {
    fn start(&self) -> Result<(), ManagerError>;
    fn stop(&self) -> Result<(), ManagerError>;
    async fn tick_all(&self) -> Result<Vec<TickOutcome>, ManagerError>;
}
```

### 5.3 AxumJpaViewManager / ActixJpaViewManager

```rust
pub struct AxumJpaViewManager<ES, PA, PS, ED> {
    event_store: Arc<ES>,
    projection_admin: Arc<PA>,
    projection_service: Arc<PS>,
    event_decoder: Arc<ED>,
    views: Vec<Arc<dyn View<...>>>,
    scheduler: JobScheduler,
    locks: HashMap<String, Arc<Semaphore>>,
}

impl<ES, PA, PS, ED> ViewManager for AxumJpaViewManager<ES, PA, PS, ED>
where ES: EventStore + 'static, ...
{ /* CRON + ViewProjector.tick */ }
```

行为契约与 Java 一致：

- 启动时注册 CRON 任务（每个 view 一个）；
- CRON 触发 → `tick(view)` → `ViewProjector.tick` → `View.handle_events`；
- 写位置前保持事务原子性；
- `stop()` 取消所有 CRON 任务并等待 inflight tick 完成。

## 6. 测试覆盖

| 测试 | 覆盖 |
|---|---|
| `event_store_config_test.rs` | env 解析、TLS / port / host 校验 |
| `query_projection_position_test.rs` | 位置持久化、并发安全 |
| `query_projection_service_test.rs` (axum) | 位置读写、reset |
| `query_projection_position_repository_test.rs` (actix) | 同上 |
| `axum_jpa_view_manager_test.rs` / `actix_jpa_view_manager_test.rs` | CRON 注册、tick_all、start/stop、内存 EventStore 投影闭环 |

## 7. 完成定义

- 20 个 `.rs` 文件齐全
- 内存 EventStore 投影闭环测试通过：写事件 → tick → view 收到事件且位置前进 → 二次 tick 不重复
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` 通过
- `tokio-cron-scheduler` 注入，便于测试替换

## 8. 与 Java 的差异

| 差异 | 解释 |
|---|---|
| Spring Boot ↔ Axum | Spring MVC 注解 → Axum handler；`@Value` → `from_env` |
| Quarkus ↔ Actix | CDI `@ObservesAsync` → `tokio::spawn`；`@Startup` → `lifecycle` |
| Quarkus 把 ProjectionService 合并 | Rust 拆为 `query_projection_position_repository.rs` |
| `tokio-cron-scheduler` 取代 Quarkus `@Scheduled` | 行为一致（cron 表达式 + 时区） |
| Actix / Axum 内部不互相依赖 | adapter 层互不引用 |