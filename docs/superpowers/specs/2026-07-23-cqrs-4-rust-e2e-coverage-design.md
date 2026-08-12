# cqrs-4-rust 端到端测试与覆盖率设计

- 日期: 2026-07-23
- 范围: `crates/test/{support,actix,axum}/` + `xtask/` (33 个迁移职责)
- 状态: 容器仅验证配置契约；真实 Docker / KurrentDB / MariaDB 链路仍 defer

## 1. 背景

Java 版在 `test/helper`、`test/quarkus`、`test/springboot` 三个模块提供端到端集成测试：构造测试用 `Person` 领域模型，启动真实 KurrentDB 与 MariaDB 容器，跑 HTTP → Command → EventStore → Projection → Query 的完整链路。

迁移必须：

1. 保留共享测试 helper；
2. 把 Actix / Axum 两套集成测试一比一复制；
3. 用 `xtask` 统一编排覆盖率，禁止新增伪业务 crate 凑数。

## 2. 目标

1. 端到端测试资源必须隔离端口和数据库；异步测试必须有有界超时。
2. 覆盖率聚合由 `xtask` 调用 `cargo llvm-cov` 完成。
3. `TestHelper` 必须真实实现 EventStoreDB / MariaDB 镜像、端口、环境、凭据、健康检查规范；容器启动本身留给真实环境验证。
4. 集成测试在缺少容器时跳过（不报错），CI 启用容器时强制运行。

## 3. 非目标

- 不实现 Testcontainers 替代品（用现成 crate）。
- 不替换 `wiremock` / `mockito` 等 HTTP mock。
- 不替换 `testcontainers-rs`。

## 4. 模块布局

```
crates/test/
├── support/                      # 源 test/helper (1)
│   └── src/test_helper.rs
├── actix/                        # 源 test/quarkus (18)
│   ├── src/
│   │   ├── app/
│   │   │   ├── kurrent_db_wrapper.rs
│   │   │   ├── person_resource.rs
│   │   │   ├── actix_app.rs
│   │   │   └── actix_factory.rs
│   │   ├── model/
│   │   │   ├── abstract_persons_view.rs
│   │   │   ├── gen_person_created_event.rs
│   │   │   ├── gen_person_id.rs
│   │   │   ├── gen_person_name.rs
│   │   │   └── person_entity.rs
│   │   ├── view/persons_view.rs
│   │   └── generated/
│   │       ├── person_created_event.rs
│   │       ├── person_id.rs
│   │       └── person_name.rs
│   └── tests/  (5)
│       ├── eventstore_resource.rs
│       ├── maria_db_resource.rs
│       ├── actix_app_test.rs
│       ├── actix_factory_test.rs
│       └── actix_test_helper.rs
└── axum/                         # 源 test/springboot (14)
    ├── src/
    │   ├── app/
    │   │   ├── person_resource.rs
    │   │   ├── axum_app.rs
    │   │   ├── axum_config.rs
    │   │   └── test_model_serde_module.rs
    │   ├── model/
    │   │   ├── gen_person_created_event.rs
    │   │   ├── gen_person_id.rs
    │   │   ├── gen_person_name.rs
    │   │   └── person_entity.rs
    │   ├── view/persons_view.rs
    │   └── generated/
    │       ├── person_created_event.rs
    │       ├── person_id.rs
    │       └── person_name.rs
    └── tests/  (2)
        ├── axum_app_test.rs
        └── axum_test_helper.rs

xtask/
└── src/
    └── coverage.rs               # 源 jacoco/Dummy.java
```

## 5. TestHelper 契约

```rust
pub struct TestHelper {
    event_store: EventStoreConfig,    // 容器化 KurrentDB 配置
    maria_db: MariaDbConfig,         // 容器化 MariaDB 配置
}

impl TestHelper {
    pub fn new() -> Self;            // 默认 127.0.0.1，端口从 env 读取
    pub fn from_env() -> Result<Self, HelperError>;
    pub async fn wait_for_ready(&self, timeout: Duration) -> Result<(), HelperError>;
    pub async fn reset_state(&self) -> Result<(), HelperError>;
}
```

实际容器启动留给 CI / 本地 docker-compose 流程；`TestHelper` 只读取端口、env、凭据、健康检查 URL。

## 6. 端到端测试场景

| 场景 | 验证 |
|---|---|
| `actix_app_test.rs` | POST /persons → 201，GET /persons/{id} → 200，DELETE /persons/{id} → 204 |
| `axum_app_test.rs` | 同上（Axum 版本） |
| `actix_factory_test.rs` / `axum_factory_test.rs` | 启动 / 关闭、绑定端口 |
| `eventstore_resource.rs` | KurrentDB 容器启动契约 |
| `maria_db_resource.rs` | MariaDB 容器启动契约 |

异步测试超时统一为 30 秒（可配置）。

## 7. 覆盖率门禁

`xtask/src/coverage.rs` 实现：

```rust
pub fn coverage_report() -> Result<CoverageReport, CoverageError> {
    // cargo llvm-cov --workspace --all-features --lcov --output-path coverage.lcov
    // 解析 lcov，断言每个 crate 行覆盖 >= 阈值
}
```

阈值由 `xtask/src/config.rs` 配置（默认 80%，发布前 90%）。

## 8. 完成定义

- 33 个迁移职责齐全（1 + 18 + 14）
- `cargo test --workspace --all-targets --all-features` 通过
- 容器缺失时集成测试跳过而非失败
- `cargo llvm-cov --workspace --all-features --lcov` 输出 ≥ 阈值

## 9. 与 Java 的差异

| 差异 | 解释 |
|---|---|
| Java WireMock → Rust `wiremock` crate | HTTP mock 行为等价 |
| Java `@QuarkusTest` → Rust `#[actix_rt::test]` / `#[tokio::test]` | 异步测试入口 |
| JaCoCo → `cargo llvm-cov` | 覆盖率工具替换 |
| Docker Testcontainers 启动放 CI | 本地可选 |