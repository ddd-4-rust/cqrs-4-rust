# Phase 5 — 端到端测试与覆盖率实施计划（33 个职责）

> 母计划: [`./2026-07-23-cqrs-4-rust-140-file-migration.md`](./2026-07-23-cqrs-4-rust-140-file-migration.md)
> 设计: [`../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md`](../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md)

---

## Task 5.1: 创建 test/support / test/actix / test/axum / xtask 骨架

- [ ] **Step 1: 注册 crate 到 workspace**

```toml
[workspace]
members = [
    "crates/core", "crates/esc",
    "crates/serialization/serde", "crates/serialization/jaxb", "crates/serialization/jsonb",
    "crates/adapter/axum", "crates/adapter/actix",
    "crates/test/support", "crates/test/actix", "crates/test/axum",
    "xtask",
]
```

- [ ] **Step 2: 每个 crate 的 `Cargo.toml`**

```toml
# crates/test/support/Cargo.toml
[package]
name = "cqrs-4-rust-test-support"
...
[dependencies]
cqrs-4-rust-axum = { path = "../../adapter/axum", version = "0.1.0", optional = true }
cqrs-4-rust-actix = { path = "../../adapter/actix", version = "0.1.0", optional = true }
thiserror = { workspace = true }
testcontainers = { workspace = true, optional = true }

[features]
default = []
axum = ["dep:cqrs-4-rust-axum"]
actix = ["dep:cqrs-4-rust-actix"]
docker = ["dep:testcontainers"]

# crates/test/actix/Cargo.toml
[dependencies]
cqrs-4-rust-test-support = { path = "../support", version = "0.1.0", features = ["actix"] }
cqrs-4-rust-actix = { path = "../../adapter/actix", version = "0.1.0" }
actix-web = { workspace = true }

# crates/test/axum/Cargo.toml
[dependencies]
cqrs-4-rust-test-support = { path = "../support", version = "0.1.0", features = ["axum"] }
cqrs-4-rust-axum = { path = "../../adapter/axum", version = "0.1.0" }
axum = { workspace = true }
tower = { workspace = true, features = ["util"] }

# xtask/Cargo.toml
[package]
name = "xtask"
...
[[bin]]
name = "xtask"
path = "src/main.rs"
```

- [ ] **Step 3: 验证骨架可编译**

```bash
cargo check --workspace --all-targets --all-features
```

- [ ] **Step 4: 提交**

```bash
git add crates/test xtask
git commit -m "feat(test+xtask): skeleton crates for e2e + coverage"
```

---

## Task 5.2: 实现 `TestHelper`

**Files:**
- `crates/test/support/src/test_helper.rs`
- `crates/test/support/src/lib.rs`

- [ ] **Step 1: 写失败测试**

```rust
#[test]
fn helper_reads_defaults() {
    let h = TestHelper::new();
    assert_eq!(h.event_store.host(), "127.0.0.1");
    assert_eq!(h.event_store.port(), 2113);
}

#[test]
fn helper_from_env_overrides() {
    std::env::set_var("EVENT_STORE_PORT", "12345");
    let h = TestHelper::from_env().unwrap();
    assert_eq!(h.event_store.port(), 12345);
}
```

- [ ] **Step 2: 实现 `TestHelper`**

```rust
pub struct TestHelper {
    pub event_store: EventStoreConfig,
    pub maria_db: MariaDbConfig,
}

impl TestHelper {
    pub fn new() -> Self { ... }
    pub fn from_env() -> Result<Self, HelperError> { ... }
    pub async fn wait_for_ready(&self, timeout: Duration) -> Result<(), HelperError> { ... }
    pub async fn reset_state(&self) -> Result<(), HelperError> { ... }
}
```

- [ ] **Step 3: 提交**

```bash
git add crates/test/support
git commit -m "feat(test-support): TestHelper with env parsing + health check contract"
```

---

## Task 5.3: 迁移 Actix 集成测试源（18 个职责）

**Files:** 见 [`../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md`](../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md) 4.2 节

- [ ] **Step 1: 实现 `app/{kurrent_db_wrapper,person_resource,actix_app,actix_factory}.rs`**

- [ ] **Step 2: 实现 `model/{abstract_persons_view,gen_person_*,person_entity}.rs`**

- [ ] **Step 3: 实现 `view/persons_view.rs`**

- [ ] **Step 4: 实现 `generated/{person_created_event,person_id,person_name}.rs`**

- [ ] **Step 5: 实现 `tests/{eventstore_resource,maria_db_resource,actix_app_test,actix_factory_test,actix_test_helper}.rs`**

- [ ] **Step 6: 容器缺失时跳过（`#[ignore = "needs container"]`）**

- [ ] **Step 7: 提交**

```bash
git add crates/test/actix
git commit -m "feat(test-actix): 18 source + test files"
```

---

## Task 5.4: 迁移 Axum 集成测试源（14 个职责）

**Files:** 见 [`../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md`](../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md) 4.3 节

- [ ] **Step 1: 实现 `app/{person_resource,axum_app,axum_config,test_model_serde_module}.rs`**

- [ ] **Step 2: 实现 `model/{gen_person_*,person_entity}.rs`**

- [ ] **Step 3: 实现 `view/persons_view.rs`**

- [ ] **Step 4: 实现 `generated/{person_created_event,person_id,person_name}.rs`**

- [ ] **Step 5: 实现 `tests/{axum_app_test,axum_test_helper}.rs`**

- [ ] **Step 6: 容器缺失时跳过**

- [ ] **Step 7: 提交**

```bash
git add crates/test/axum
git commit -m "feat(test-axum): 14 source + test files"
```

---

## Task 5.5: 实现 `xtask/src/coverage.rs`（对应 `jacoco/Dummy.java`）

**Files:**
- `xtask/src/main.rs`
- `xtask/src/coverage.rs`
- `xtask/src/config.rs`

- [ ] **Step 1: 实现 `xtask coverage` 子命令**

```rust
// xtask/src/coverage.rs
use std::process::Command;
use crate::config::Config;

pub fn run(cfg: &Config) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(["llvm-cov", "--workspace", "--all-features", "--lcov", "--output-path", "coverage.lcov"])
        .status()?;
    assert!(status.success());
    let report = parse_lcov("coverage.lcov")?;
    assert!(report.line_coverage() >= cfg.coverage_threshold);
    Ok(())
}
```

- [ ] **Step 2: 实现 `xtask check-serialization-protocol`**

- [ ] **Step 3: 实现 `xtask check-migration-parity`（可调用 `tools/check_migration_parity.sh`）**

- [ ] **Step 4: 提交**

```bash
git add xtask
git commit -m "feat(xtask): coverage + serialization protocol + migration parity"
```

---

## Task 5.6: 终态校验

- [ ] **Step 1: 全量门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
./tools/check_migration_parity.sh
```

- [ ] **Step 2: 覆盖率门禁**

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path coverage.lcov
cargo run -p xtask -- coverage --threshold 80
```

- [ ] **Step 3: 发布前检查**

```bash
cargo tree --duplicates
cargo deny check
cargo audit
```

- [ ] **Step 4: 提交 Phase 5 完成标记**

```bash
git commit --allow-empty -m "chore(e2e): Phase 5 migration complete"
```

---

## 完成标志

- 1 + 18 + 14 = 33 个迁移职责齐全
- TestHelper 契约测试通过
- 容器缺失时集成测试优雅跳过
- 覆盖率 ≥ 阈值
- 全部门禁通过