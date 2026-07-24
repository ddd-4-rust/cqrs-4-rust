# cqrs-4-rust 迁移状态

> 基线：`cqrs-4-java` 0.6.0 (`1e9d64f58a11f2bc978ce687d98eba811eb9b022`)

完成率不再按“创建了多少文件”计算。一个文件只有同时满足实现、行为测试、文档和质量门禁后才记为完成。

| 模块 | 目标文件 | 当前文件 | 已验收 | 当前状态 |
|---|---:|---:|---:|---|
| core | 21 | 21 | 0 | 文件已对齐；路由、Adler-32 和线协议测试已补，仍需 JPA context/完整异常语义复核 |
| esc | 6 | 6 | 1 | 映射文件 6/6；另增 `ProjectionAdmin` / `EventDecoder` / `ViewProjector` 共享引擎（非 Java 文件映射项）。dispatcher 与内存投影 tick 已验收 |
| serde（源 jackson） | 22 | 22 | 0 | 8 个生产职责与 14 个测试/fixture 职责到位；已修正 command 的 Java 连字符字段协议，全部测试与 Clippy 通过 |
| jaxb | 15 | 15 | 0 | 5 个生产职责与 10 个测试/fixture 职责到位；真实 XML 编解码和线协议测试通过，仍需与 Java 黄金 XML 样本做最终验收 |
| jsonb | 22 | 22 | 0 | 8 个生产职责和 14 个对应测试职责已到位；Serde 动态字段协议、错误分支与 inventory 编译期注册测试通过 |
| actix（源 Quarkus） | 10 | 10 | 1 | ViewManager 已接入 `tokio-cron-scheduler` + 共享 `ViewProjector`；内存 EventStore 下位置前进/幂等/启停已验收。仍 defer：Kurrent 真投影、MariaDB 持久化 |
| axum（源 Spring Boot） | 10 | 10 | 1 | ViewManager 已接入 CRON + `ViewProjector`；内存 EventStore 投影闭环已验收。仍 defer：Kurrent 真投影、MariaDB 持久化 |
| coverage | 1 | 1 | 0 | 已映射为可执行的 `xtask/src/coverage.rs` |
| test/helper | 1 | 1 | 0 | EventStoreDB/MariaDB 镜像、端口、环境、凭据与健康检查规范已实现；实际容器启动留给端到端验收 |
| test/actix（源 Quarkus） | 18 | 18 | 0 | 生成模型、工厂、投影、Actix HTTP 和 5 个测试职责到位；容器仅验证配置契约，尚未执行真实 Docker/KurrentDB/MariaDB 链路 |
| test/axum（源 Spring Boot） | 14 | 14 | 0 | 生成模型、配置、Serde 模块、投影、Axum HTTP 和 2 个测试职责到位；尚未执行真实 Docker/KurrentDB/MariaDB 链路 |
| **合计** | **140** | **140** | **3** | **100% 文件职责到位；适配器投影调度已可本地使用，外部 ES/DB e2e 仍 defer** |

## 可用性进展（2026-07-23）

读侧投影闭环现已可在进程内落地使用：

1. `cqrs-4-rust-esc`：`ProjectionStreamId`、`ProjectionAdmin`（含 `MemoryProjectionAdmin`）、`EventDecoder` / `RegistryEventDecoder`、`ViewProjector` / `ManagedView`
2. `AxumJpaViewManager` / `ActixJpaViewManager`：注入 `EventStore` + `ProjectionAdmin` + `ProjectionService` + `EventDecoder`；`start()` 注册 CRON，`tick_all()` 供测试/手动推进，`stop()` 取消任务并 shutdown
3. 行为测试（内存 EventStore）：写事件 → tick → view 收到事件且位置前进 → 二次 tick 不重复

仍 defer（不影响库级本地使用）：

- 真实 `KurrentDB` gRPC `ProjectionAdmin` 客户端
- MariaDB / SQL 投影位置持久化
- Docker 容器端到端链路

## 当前验证快照

- 本仓库 `crates/` 与 `xtask/` 全部 Rust 源码直接执行 `rustfmt --check`：通过。
- `cargo metadata --no-deps`：通过，确认 12 个 workspace member 均可解析。
- `cargo tree -p cqrs-4-rust --all-features`：通过，facade 的可选依赖方向正确且无环。
- `cargo check/test/clippy -p cqrs-4-rust-core -p cqrs-4-rust-esc -p cqrs-4-rust`：通过。
- `cargo test -p cqrs-4-rust-esc -p cqrs-4-rust-axum -p cqrs-4-rust-actix --all-features`：通过（含投影闭环行为测试）。
- `cargo clippy -p cqrs-4-rust-esc -p cqrs-4-rust-axum -p cqrs-4-rust-actix --all-targets --all-features -- -D warnings`：通过。
- `cargo check --workspace --all-targets --all-features`：通过。
- `cargo test --workspace --all-features`：通过，包括 Serde、JAXB、JSON-B、Actix 与 Axum in-process 端到端场景。
- `docs/migration/file_mapping.csv`：已建立 140 条双向唯一映射，当前 `present=140`、`planned=0`。
- `tools/check_migration_parity.sh`：通过；Java 迁移文件 140、映射 140、Rust mapped 文件 140。

详细验收口径见 [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)。
