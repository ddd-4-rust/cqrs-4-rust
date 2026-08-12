# cqrs-4-rust 一比一迁移到 Rust 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 Java 版 `cqrs-4-java 0.6.0`（提交 `1e9d64f58a11f2bc978ce687d98eba811eb9b022`）一比一迁移到 Rust，产出 140 个职责一一对应的 `.rs` 文件，行为契约、错误语义、序列化协议与测试场景与 Java 等价。

**Architecture:** Cargo virtual workspace，5 个核心 crate（`core`、`esc`、3 线序列化、2 框架适配、3 测试包）+ `cqrs` facade + `xtask` 工具链。详见 [`../specs/2026-07-23-cqrs-4-rust-architecture-design.md`](../specs/2026-07-23-cqrs-4-rust-architecture-design.md)。

**Tech Stack:** Rust 2024 edition、resolver 3、MSRV 1.88、Tokio、Axum、Actix-web、Serde、quick-xml、inventory、sqlx、tokio-cron-scheduler、testcontainers、cargo llvm-cov。

参考设计文档：

- [`../specs/2026-07-23-cqrs-4-rust-architecture-design.md`](../specs/2026-07-23-cqrs-4-rust-architecture-design.md)
- [`../specs/2026-07-23-cqrs-4-rust-core-design.md`](../specs/2026-07-23-cqrs-4-rust-core-design.md)
- [`../specs/2026-07-23-cqrs-4-rust-esc-design.md`](../specs/2026-07-23-cqrs-4-rust-esc-design.md)
- [`../specs/2026-07-23-cqrs-4-rust-serialization-design.md`](../specs/2026-07-23-cqrs-4-rust-serialization-design.md)
- [`../specs/2026-07-23-cqrs-4-rust-framework-adapter-design.md`](../specs/2026-07-23-cqrs-4-rust-framework-adapter-design.md)
- [`../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md`](../specs/2026-07-23-cqrs-4-rust-e2e-coverage-design.md)

迁移事实账本：[`./2026-07-23-cqrs-4-rust-migration-accounting.csv`](./2026-07-23-cqrs-4-rust-migration-accounting.csv)。

---

## 文件结构总览

```
cqrs-4-rust/
├── Cargo.toml                          # virtual workspace
├── crates/
│   ├── cqrs/                           # feature-gated public facade
│   ├── core/                           # 21 个迁移职责
│   ├── esc/                            # 6 个迁移职责 + 共享引擎
│   ├── serialization/
│   │   ├── serde/                      # 22 个迁移职责
│   │   ├── jaxb/                       # 15 个迁移职责
│   │   └── jsonb/                      # 22 个迁移职责
│   ├── adapter/
│   │   ├── axum/                       # 10 个迁移职责
│   │   └── actix/                      # 10 个迁移职责
│   └── test/
│       ├── support/                    # 1 个迁移职责
│       ├── actix/                      # 18 个迁移职责
│       └── axum/                       # 14 个迁移职责
├── xtask/                              # 迁移、对账、覆盖率自动化
└── docs/superpowers/                   # 本规范
    ├── plans/                          # 当前目录
    └── specs/                          # 设计文档
```

---

## Task 0: 基线与对账机制

- [ ] **Step 1: 在仓库根建立权威对账脚本**

创建 `tools/check_migration_parity.sh`：

```bash
#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CSV="$REPO_ROOT/docs/superpowers/plans/2026-07-23-cqrs-4-rust-migration-accounting.csv"

# 1. Java 文件数（排除 Maven Wrapper）
java_count=$(find "$REPO_ROOT" -path "*/cqrs-4-java/*" -name "*.java" \
    ! -path "*/.mvn/wrapper/*" | wc -l | tr -d ' ')

# 2. CSV 映射数（不含 header）
map_count=$(($(wc -l < "$CSV") - 1))

# 3. Rust mapped 文件存在数
rust_present=$(awk -F, 'NR>1 && $3=="present" {print $2}' "$CSV" \
    | while read p; do [ -f "$REPO_ROOT/$p" ] && echo 1; done | wc -l | tr -d ' ')

echo "java: $java_count  mapped: $map_count  rust_present: $rust_present"
[ "$java_count" -eq 140 ] || { echo "FAIL: java != 140"; exit 1; }
[ "$map_count" -eq 140 ] || { echo "FAIL: csv mapping != 140"; exit 1; }
[ "$rust_present" -eq 140 ] || { echo "FAIL: rust present != 140"; exit 1; }
echo "OK"
```

- [ ] **Step 2: 设置脚本可执行并加入 CI**

```bash
chmod +x tools/check_migration_parity.sh
git add tools/check_migration_parity.sh
git commit -m "ci: add migration parity check script"
```

- [ ] **Step 3: 在 `.github/workflows/ci.yml` 中调用对账**

新增步骤：

```yaml
- name: Migration parity
  run: ./tools/check_migration_parity.sh
```

期望：`OK`。

---

## Task 1: Phase 1 — Core（21 个职责）

**Files:** 见 [`2026-07-23-cqrs-4-rust-core-phase.md`](./2026-07-23-cqrs-4-rust-core-phase.md)

- [ ] **Step 1: 阅读 Phase 1 子计划**

按 [`./2026-07-23-cqrs-4-rust-core-phase.md`](./2026-07-23-cqrs-4-rust-core-phase.md) 的 Task 列表逐项推进。

- [ ] **Step 2: Phase 1 全部勾选完成后，运行门禁**

```bash
cargo test -p cqrs-4-rust-core --all-features
cargo clippy -p cqrs-4-rust-core --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

期望：全部通过。

- [ ] **Step 3: 验证迁移对账**

```bash
./tools/check_migration_parity.sh
```

期望：仍然 `java: 140 mapped: 140 rust_present: 140`。

- [ ] **Step 4: 提交**

```bash
git add crates/core docs/superpowers tools/check_migration_parity.sh
git commit -m "feat(core): Phase 1 migration complete (21/21)"
```

---

## Task 2: Phase 2 — ESC（6 个职责 + 共享引擎）

**Files:** 见 [`2026-07-23-cqrs-4-rust-esc-phase.md`](./2026-07-23-cqrs-4-rust-esc-phase.md)

- [ ] **Step 1: 阅读 Phase 2 子计划**

按 [`./2026-07-23-cqrs-4-rust-esc-phase.md`](./2026-07-23-cqrs-4-rust-esc-phase.md) 的 Task 列表逐项推进。

- [ ] **Step 2: 内存投影闭环测试通过**

```bash
cargo test -p cqrs-4-rust-esc --all-features
```

期望：`projection_tick_advances_position_and_dispatches_events` 等用例全部通过。

- [ ] **Step 3: 门禁**

```bash
cargo clippy -p cqrs-4-rust-esc --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
./tools/check_migration_parity.sh
```

期望：全部通过。

- [ ] **Step 4: 提交**

```bash
git add crates/esc docs/superpowers
git commit -m "feat(esc): Phase 2 migration complete (6/6) + shared engine"
```

---

## Task 3: Phase 3 — 序列化（59 个职责）

**Files:** 见 [`2026-07-23-cqrs-4-rust-serialization-phase.md`](./2026-07-23-cqrs-4-rust-serialization-phase.md)

- [ ] **Step 1: 阅读 Phase 3 子计划**

按 [`./2026-07-23-cqrs-4-rust-serialization-phase.md`](./2026-07-23-cqrs-4-rust-serialization-phase.md) 的 Task 列表逐项推进 serde / jaxb / jsonb 三线。

- [ ] **Step 2: 协议字段一致性**

```bash
cargo test --workspace --all-features
cargo run -p xtask -- check-serialization-protocol
```

期望：JSON 字段名 (`type` / `code` / `message` / `data-class` / `data-element`) 与 Java 一致。

- [ ] **Step 3: JAXB 黄金 XML 样本回归**

```bash
cargo test -p cqrs-4-rust-jaxb --all-features --test golden_xml
```

期望：所有 JAXB fixture XML 解析/输出与 Java 等价。

- [ ] **Step 4: 门禁**

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
./tools/check_migration_parity.sh
```

- [ ] **Step 5: 提交**

```bash
git add crates/serialization docs/superpowers
git commit -m "feat(serialization): Phase 3 migration complete (59/59)"
```

---

## Task 4: Phase 4 — 框架适配（20 个职责）

**Files:** 见 [`2026-07-23-cqrs-4-rust-framework-adapter-phase.md`](./2026-07-23-cqrs-4-rust-framework-adapter-phase.md)

- [ ] **Step 1: 阅读 Phase 4 子计划**

按 [`./2026-07-23-cqrs-4-rust-framework-adapter-phase.md`](./2026-07-23-cqrs-4-rust-framework-adapter-phase.md) 推进 axum / actix 两套。

- [ ] **Step 2: 内存 EventStore 投影闭环**

```bash
cargo test -p cqrs-4-rust-axum --all-features
cargo test -p cqrs-4-rust-actix --all-features
```

期望：`start() / tick_all() / stop()` 行为与 Java 一致。

- [ ] **Step 3: EventStoreConfig 校验回归**

```bash
cargo test -p cqrs-4-rust-axum --all-features --test event_store_config_test
cargo test -p cqrs-4-rust-actix --all-features --test event_store_config_test
```

期望：host / port / TLS / protocol 校验全通过。

- [ ] **Step 4: 门禁**

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
./tools/check_migration_parity.sh
```

- [ ] **Step 5: 提交**

```bash
git add crates/adapter docs/superpowers
git commit -m "feat(adapter): Phase 4 migration complete (20/20)"
```

---

## Task 5: Phase 5 — 端到端测试与覆盖率（33 个职责）

**Files:** 见 [`2026-07-23-cqrs-4-rust-e2e-coverage-phase.md`](./2026-07-23-cqrs-4-rust-e2e-coverage-phase.md)

- [ ] **Step 1: 阅读 Phase 5 子计划**

按 [`./2026-07-23-cqrs-4-rust-e2e-coverage-phase.md`](./2026-07-23-cqrs-4-rust-e2e-coverage-phase.md) 推进 test/support / test/actix / test/axum + xtask。

- [ ] **Step 2: TestHelper 契约**

```bash
cargo test -p cqrs-4-rust-test-support --all-features
```

期望：`TestHelper::new()` / `from_env()` / `wait_for_ready()` 通过契约断言。

- [ ] **Step 3: 容器缺失时集成测试跳过**

```bash
cargo test --workspace --all-targets --all-features
```

期望：在没有 Docker / KurrentDB / MariaDB 时，端到端测试输出 `skipped`，单元测试全部通过。

- [ ] **Step 4: 覆盖率门禁**

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path coverage.lcov
cargo run -p xtask -- coverage --threshold 80
```

期望：覆盖率 ≥ 阈值。

- [ ] **Step 5: 终态门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
./tools/check_migration_parity.sh
cargo tree --duplicates
cargo deny check
cargo audit
```

期望：全部通过。

- [ ] **Step 6: 提交**

```bash
git add crates/test xtask docs/superpowers
git commit -m "feat(e2e): Phase 5 migration complete (33/33)"
```

---

## Task 6: 真实 KurrentDB / MariaDB 端到端链路（持续）

> Phase 5 已通过容器缺失时跳过的策略保证本地开发可用；真实容器链路在 CI 启用 Docker 后强制运行，作为持续跟进项。

- [ ] **Step 1: 在 `.github/workflows/ci.yml` 启用 `services`**

```yaml
services:
  kurrentdb:
    image: kurrent/kurrentdb:24.10
    ports: ["2113:2113"]
  mariadb:
    image: mariadb:11
    env:
      MARIADB_ROOT_PASSWORD: root
    ports: ["3306:3306"]
```

- [ ] **Step 2: 集成测试去掉 `#[ignore]`**

把 `crates/test/{actix,axum}/tests/*.rs` 中的 `#[ignore = "needs container"]` 改为 `#[ignore = "set RUN_E2E=1"]`，仅在 CI 强制运行。

- [ ] **Step 3: 在 CI 中 `RUN_E2E=1 cargo test ...`**

- [ ] **Step 4: 真实链路接入 PostgreSQL / KurrentDB gRPC `ProjectionAdmin` 客户端**

当 `ESC` 接入真实 KurrentDB 时，`MemoryProjectionAdmin` 与 `RegistryEventDecoder` 之外提供 `KurrentProjectionAdmin` 实现。

---

## 完成标志

- 5 个 Phase 子计划全部勾选完成；
- 140 个 Rust 文件全部存在（`tools/check_migration_parity.sh` 通过）；
- 终态门禁 9 项全部通过（fmt / check / test / clippy / doc / parity / duplicates / deny / audit）；
- 真实 KurrentDB / MariaDB 端到端链路在 CI 中持续验证；
- 仓库 `docs/` 下不存在 `ARCHITECTURE.md` / `IMPLEMENTATION_PLAN.md` / `MIGRATION_STATUS.md` / `migration/` 旧目录（已统一到 `docs/superpowers/`）。