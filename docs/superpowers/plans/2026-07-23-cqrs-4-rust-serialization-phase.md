# Phase 3 — 序列化实施计划（59 个职责）

> 母计划: [`./2026-07-23-cqrs-4-rust-140-file-migration.md`](./2026-07-23-cqrs-4-rust-140-file-migration.md)
> 设计: [`../specs/2026-07-23-cqrs-4-rust-serialization-design.md`](../specs/2026-07-23-cqrs-4-rust-serialization-design.md)

---

## Task 3.1: 创建三线骨架（serde / jaxb / jsonb）

- [ ] **Step 1: 在根 `Cargo.toml` 注册三线 crate**

```toml
[workspace]
members = [
    "crates/core",
    "crates/esc",
    "crates/serialization/serde",
    "crates/serialization/jaxb",
    "crates/serialization/jsonb",
    ...
]
```

- [ ] **Step 2: 每个 crate 的 `Cargo.toml`**

```toml
# crates/serialization/serde/Cargo.toml
[package]
name = "cqrs-4-rust-serde"
...
[dependencies]
cqrs-4-rust-core = { path = "../../core", version = "0.1.0" }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
thiserror = { workspace = true }

# crates/serialization/jaxb/Cargo.toml
[dependencies]
cqrs-4-rust-core = { path = "../../core", version = "0.1.0" }
quick-xml = { workspace = true, features = ["serialize"] }
serde = { workspace = true, features = ["derive"] }
thiserror = { workspace = true }

# crates/serialization/jsonb/Cargo.toml
[dependencies]
cqrs-4-rust-core = { path = "../../core", version = "0.1.0" }
inventory = { workspace = true }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
thiserror = { workspace = true }
```

- [ ] **Step 3: 每个 crate 的 `src/lib.rs` 声明 8/5/8 个模块**

- [ ] **Step 4: 验证三线骨架可编译**

```bash
cargo check --workspace --all-targets --all-features
```

- [ ] **Step 5: 提交**

```bash
git add crates/serialization
git commit -m "feat(serialization): skeleton crates (serde/jaxb/jsonb)"
```

---

## Task 3.2: 实现协议字段常量（共享）

**Files:**
- 在三线 crate 中分别实现 `protocol.rs` 或在 `lib.rs` 中暴露常量

- [ ] **Step 1: 三线常量一致**

```rust
pub const RESULT_TYPE_FIELD: &str = "type";
pub const RESULT_CODE_FIELD: &str = "code";
pub const RESULT_MESSAGE_FIELD: &str = "message";
pub const RESULT_DATA_CLASS_FIELD: &str = "data-class";
pub const RESULT_DATA_ELEMENT_FIELD: &str = "data-element";

pub const RESULT_TYPE_OK: &str = "OK";
pub const RESULT_TYPE_WARNING: &str = "WARNING";
pub const RESULT_TYPE_ERROR: &str = "ERROR";
```

- [ ] **Step 2: 写 `protocol_test.rs` 断言**

- [ ] **Step 3: 提交**

```bash
git add crates/serialization
git commit -m "feat(serialization): protocol field constants"
```

---

## Task 3.3: 实现 `AbstractCommand` / `AbstractAggregateCommand` / `AbstractResult` / `DataResult` / `SimpleResult`（三线）

**Files（每条线 5 个文件）：**

serde: `crates/serialization/serde/src/{abstract_command,abstract_aggregate_command,abstract_result,data_result,simple_result}.rs`
jaxb: `crates/serialization/jaxb/src/{...}.rs`
jsonb: `crates/serialization/jsonb/src/{...}.rs`

- [ ] **Step 1: 写失败测试（serde 线）**

`crates/serialization/serde/tests/abstract_command_test.rs`：

```rust
use cqrs_4_rust_serde::{AbstractCommand, AbstractAggregateCommand};
use cqrs_4_rust_core::Event;

struct MyCmd { id: String, name: String }
impl Event for MyCmd {
    fn event_type(&self) -> &'static str { "MyCmd" }
}

#[test]
fn abstract_command_round_trip_json() {
    let cmd = MyCmd { id: "p-1".into(), name: "Alice".into() };
    let json = serde_json::to_string(&AbstractCommand::from(&cmd)).unwrap();
    assert!(json.contains("\"type\":\"MyCmd\""));
    assert!(json.contains("\"id\":\"p-1\""));
}
```

- [ ] **Step 2: 实现 serde 线 5 个类型**

- [ ] **Step 3: 实现 jaxb 线 5 个类型（XML）**

- [ ] **Step 4: 实现 jsonb 线 5 个类型**

- [ ] **Step 5: 写测试覆盖**

每个类型至少 1 个 round-trip 测试 + 1 个 error 分支测试。

- [ ] **Step 6: 提交**

```bash
git add crates/serialization
git commit -m "feat(serialization): AbstractCommand/DataResult/SimpleResult across serde/jaxb/jsonb"
```

---

## Task 3.4: 实现 serde 线适配（`cqrs_4_serde_module` / `data_result_deserializer` / `data_result_serializer`）

**Files:**
- `crates/serialization/serde/src/cqrs_4_serde_module.rs`
- `crates/serialization/serde/src/data_result_deserializer.rs`
- `crates/serialization/serde/src/data_result_serializer.rs`

- [ ] **Step 1: 实现 `DataResultJacksonSerializer` → `data_result_serializer.rs`**

```rust
pub fn serialize<S, D>(value: &CqrsResult<D>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer, D: Serialize {
    let mut state = serializer.serialize_struct("DataResult", 5)?;
    state.serialize_field("type", match value.result_type {
        ResultType::Ok => "OK", ResultType::Warning => "WARNING", ResultType::Error => "ERROR",
    })?;
    state.serialize_field("code", value.error_code.as_deref().unwrap_or(""))?;
    state.serialize_field("message", value.error_message.as_deref().unwrap_or(""))?;
    state.serialize_field("data-class", "")?;     // 由调用方注入
    state.serialize_field("data-element", &value.data)?;
    state.end()
}
```

- [ ] **Step 2: 反向实现 deserializer**

- [ ] **Step 3: `cqrs_4_serde_module` 注册 adapter**

- [ ] **Step 4: 测试覆盖**

- [ ] **Step 5: 提交**

```bash
git add crates/serialization/serde/src/cqrs_4_serde_module.rs crates/serialization/serde/src/data_result_deserializer.rs crates/serialization/serde/src/data_result_serializer.rs
git commit -m "feat(serialization): serde module + DataResult (de)serializer"
```

---

## Task 3.5: 实现 jsonb 线注册机制（`JsonbRegistry` / `JandexJsonbRegistry`）

**Files:**
- `crates/serialization/jsonb/src/jsonb_registry.rs`
- `crates/serialization/jsonb/src/jandex_jsonb_registry.rs`
- `crates/serialization/jsonb/src/data_result_jsonb_adapter.rs`

- [ ] **Step 1: 实现 `JsonbRegistry` trait + 内存注册表**

```rust
inventory::collect!(JsonbRegistryEntry);

pub trait JsonbRegistryEntryTrait {
    fn event_type(&self) -> &'static str;
    fn to_json(&self, value: &dyn std::any::Any) -> Result<serde_json::Value, JsonbError>;
}
```

- [ ] **Step 2: 实现 `JandexJsonbRegistry`（编译期镜像）**

- [ ] **Step 3: 实现 `DataResultJsonbAdapter`**

- [ ] **Step 4: 测试覆盖**

- [ ] **Step 5: 提交**

```bash
git add crates/serialization/jsonb
git commit -m "feat(jsonb): compile-time registry via inventory + DataResultJsonbAdapter"
```

---

## Task 3.6: 实现测试 fixture / 测试套件

**Files（每个 crate 的 `tests/` 共 38 个测试文件）：**

按 Java 测试文件 1:1 复制为 Rust snake_case 测试文件。

- [ ] **Step 1: 写每个测试文件的失败用例**

- [ ] **Step 2: 实现测试 fixture（`a_created_event` / `a_id` / `b_id` / `c_id` / `invoice` / `my_id_factory` / `test_utils`）**

- [ ] **Step 3: 实现 `architecture_test.rs` 守门（trait 形态合规）**

- [ ] **Step 4: 实现 `base_test.rs` 测试基类**

- [ ] **Step 5: 提交**

```bash
git add crates/serialization
git commit -m "test(serialization): phase 3 test suite (38 fixture files)"
```

---

## Task 3.7: 终态校验

- [ ] **Step 1: 全量门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
./tools/check_migration_parity.sh
```

- [ ] **Step 2: JAXB 黄金 XML 样本回归**

```bash
cargo test -p cqrs-4-rust-jaxb --all-features
```

期望：所有 JAXB fixture 与 Java 黄金值一致。

- [ ] **Step 3: 协议字段 lint**

```bash
cargo run -p xtask -- check-serialization-protocol
```

期望：JSON 字段名 (`type` / `code` / `message` / `data-class` / `data-element`) 与 Java 一致。

- [ ] **Step 4: 提交 Phase 3 完成标记**

```bash
git commit --allow-empty -m "chore(serialization): Phase 3 migration complete"
```

---

## 完成标志

- 22 + 15 + 22 = 59 个迁移职责齐全
- JSON / XML 协议字段与 Java 黄金值一致
- `inventory` 注册机制替代 Jandex
- `tools/check_migration_parity.sh` 通过