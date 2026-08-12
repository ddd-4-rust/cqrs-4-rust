# Phase 1 — Core 实施计划（21 个职责）

> 母计划: [`./2026-07-23-cqrs-4-rust-140-file-migration.md`](./2026-07-23-cqrs-4-rust-140-file-migration.md)
> 设计: [`../specs/2026-07-23-cqrs-4-rust-core-design.md`](../specs/2026-07-23-cqrs-4-rust-core-design.md)

---

## Task 1.1: 创建 `crates/core` 骨架

- [ ] **Step 1: 创建 `crates/core/Cargo.toml`**

```toml
[package]
name = "cqrs-4-rust-core"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true

[dependencies]
thiserror = { workspace = true }
inventory = { workspace = true, optional = true }

[features]
default = []
registry = ["dep:inventory"]
```

在根 `Cargo.toml` 的 `[workspace] members` 中加入 `crates/core`。

- [ ] **Step 2: 创建 `src/lib.rs` 骨架**

```rust
//! cqrs-4-rust-core
//!
//! 对应 Java `cqrs-4-java/core` 的 21 个迁移职责。

pub mod abstract_multi_command_executor;
pub mod aggregate_command;
pub mod command;
pub mod command_executor;
pub mod command_execution_failed_error;
pub mod cqrs_utils;
pub mod jpa_event_handler;
pub mod jpa_view;
pub mod multi_command_executor;
pub mod result;
pub mod result_type;
pub mod to_result_capable;
pub mod url_param_entity_id_path_not_equals_cmd_error;
pub mod view;
```

- [ ] **Step 3: 验证骨架可编译**

```bash
cargo check -p cqrs-4-rust-core --all-targets --all-features
```

期望：通过（每个 module 文件先空实现）。

- [ ] **Step 4: 提交**

```bash
git add crates/core
git commit -m "feat(core): skeleton crate with module declarations"
```

---

## Task 1.2: 实现 `Command` / `AggregateCommand` / `Event`

**Files:**
- `crates/core/src/command.rs`
- `crates/core/src/aggregate_command.rs`

- [ ] **Step 1: 写失败测试**

`crates/core/tests/command_test.rs`：

```rust
use cqrs_4_rust_core::{Command, AggregateCommand, Event};

struct CreatePersonCommand { root_id: String, entity_id: Option<String> }
impl Event for CreatePersonCommand {
    fn event_type(&self) -> &'static str { "CreatePersonCommand" }
}
impl Command for CreatePersonCommand {}
impl AggregateCommand<String, String> for CreatePersonCommand {
    fn aggregate_root_id(&self) -> &String { &self.root_id }
    fn aggregate_entity_id(&self) -> Option<&String> { self.entity_id.as_ref() }
}

#[test]
fn command_carries_root_id_and_optional_entity_id() {
    let cmd = CreatePersonCommand { root_id: "p-1".into(), entity_id: Some("e-1".into()) };
    assert_eq!(cmd.aggregate_root_id(), "p-1");
    assert_eq!(cmd.aggregate_entity_id(), Some(&"e-1".to_string()));
}
```

- [ ] **Step 2: 运行测试，确认失败**

```bash
cargo test -p cqrs-4-rust-core --test command_test
```

期望：编译失败（`Command` / `AggregateCommand` / `Event` 不存在）。

- [ ] **Step 3: 实现 `event.rs` / `command.rs` / `aggregate_command.rs`**

```rust
// crates/core/src/event.rs
pub trait Event {
    fn event_type(&self) -> &'static str;
}

// crates/core/src/command.rs
use super::event::Event;
pub trait Command: Event {}

// crates/core/src/aggregate_command.rs
pub trait AggregateCommand<RootId, EntityIdType> {
    fn aggregate_root_id(&self) -> &RootId;
    fn aggregate_entity_id(&self) -> Option<&EntityIdType>;
}
```

并在 `lib.rs` 加 `pub mod event;`。

- [ ] **Step 4: 再次运行测试，确认通过**

```bash
cargo test -p cqrs-4-rust-core --test command_test
```

期望：1 test passed。

- [ ] **Step 5: 提交**

```bash
git add crates/core/src/event.rs crates/core/src/command.rs crates/core/src/aggregate_command.rs crates/core/src/lib.rs crates/core/tests/command_test.rs
git commit -m "feat(core): introduce Event/Command/AggregateCommand traits"
```

---

## Task 1.3: 实现 `Result` / `ResultType` / `ToResultCapable` / `CqrsResult`

**Files:**
- `crates/core/src/result_type.rs`
- `crates/core/src/result.rs`
- `crates/core/src/to_result_capable.rs`

- [ ] **Step 1: 写失败测试**

`crates/core/tests/result_test.rs`：

```rust
use cqrs_4_rust_core::{CqrsResult, ResultType, ToResultCapable};

#[test]
fn ok_result_carries_data() {
    let r: CqrsResult<i32> = CqrsResult::ok(42);
    assert_eq!(r.result_type, ResultType::Ok);
    assert_eq!(r.data, Some(42));
    assert_eq!(r.error_code, None);
    assert_eq!(r.error_message, None);
}

#[test]
fn warning_result_carries_code_message() {
    let r: CqrsResult<i32> = CqrsResult::warning(7, "degraded");
    assert_eq!(r.result_type, ResultType::Warning);
    assert_eq!(r.data, Some(7));
    assert_eq!(r.error_code.as_deref(), Some(""));
    // warning 的 message 字段为 "degraded"
}

#[test]
fn to_result_lifts_plain_value_into_ok() {
    let r: CqrsResult<i32> = 42i32.to_result();
    assert_eq!(r.result_type, ResultType::Ok);
    assert_eq!(r.data, Some(42));
}
```

- [ ] **Step 2: 实现 `result_type.rs`**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultType { Ok, Warning, Error }
```

- [ ] **Step 3: 实现 `result.rs`**

```rust
use super::result_type::ResultType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqrsResult<D> {
    pub result_type: ResultType,
    pub data: Option<D>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

impl<D> CqrsResult<D> {
    pub fn ok(data: D) -> Self {
        Self { result_type: ResultType::Ok, data: Some(data), error_code: None, error_message: None }
    }
    pub fn warning(data: D, message: &str) -> Self {
        Self {
            result_type: ResultType::Warning,
            data: Some(data),
            error_code: None,
            error_message: Some(message.into()),
        }
    }
    pub fn error(code: &str, message: &str) -> Self {
        Self {
            result_type: ResultType::Error,
            data: None,
            error_code: Some(code.into()),
            error_message: Some(message.into()),
        }
    }
}
```

- [ ] **Step 4: 实现 `to_result_capable.rs`**

```rust
use super::result::{CqrsResult, ResultType};

pub trait ToResultCapable {
    type Data;
    fn to_result(self) -> CqrsResult<Self::Data>;
}

impl<T> ToResultCapable for T {
    type Data = T;
    fn to_result(self) -> CqrsResult<Self::Data> {
        CqrsResult { result_type: ResultType::Ok, data: Some(self), error_code: None, error_message: None }
    }
}
```

- [ ] **Step 5: 运行测试，确认通过**

```bash
cargo test -p cqrs-4-rust-core --test result_test
```

- [ ] **Step 6: 提交**

```bash
git add crates/core/src/result.rs crates/core/src/result_type.rs crates/core/src/to_result_capable.rs crates/core/src/lib.rs crates/core/tests/result_test.rs
git commit -m "feat(core): add CqrsResult/ResultType/ToResultCapable"
```

---

## Task 1.4: 实现 `CommandExecutor` / `AbstractMultiCommandExecutor` / `MultiCommandExecutor`

**Files:**
- `crates/core/src/command_executor.rs`
- `crates/core/src/abstract_multi_command_executor.rs`
- `crates/core/src/multi_command_executor.rs`
- `crates/core/tests/multi_command_executor_test.rs`

- [ ] **Step 1: 写失败测试**

```rust
use cqrs_4_rust_core::*;

struct AddCmd(i32, i32);
impl Event for AddCmd { fn event_type(&self) -> &'static str { "AddCmd" } }
impl Command for AddCmd {}

struct AddExecutor;
impl CommandExecutor<(), AddCmd, i32> for AddExecutor {
    fn execute(&self, _: &(), cmd: AddCmd) -> Result<i32, CoreError> { Ok(cmd.0 + cmd.1) }
}

struct PipelineExecutor { steps: Vec<Box<dyn Fn(i32) -> i32>> }

#[test]
fn multi_command_executor_runs_pipeline() {
    let pipeline = MultiCommandExecutor::<(), AddCmd, i32>::new(
        Box::new(AddExecutor),
        vec![Box::new(|x| x * 2), Box::new(|x| x + 1)],
    );
    let r = pipeline.execute(&(), AddCmd(3, 4)).unwrap();
    assert_eq!(r, 15); // (3+4) * 2 + 1
}
```

- [ ] **Step 2: 实现 `command_executor.rs`**

```rust
use super::command::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("command execution failed: {0}")]
    ExecutionFailed(String),
    #[error("invalid state: {0}")]
    InvalidState(String),
}

pub trait CommandExecutor<Ctx, Cmd: Command, R> {
    fn execute(&self, ctx: &Ctx, cmd: Cmd) -> Result<R, CoreError>;
}
```

- [ ] **Step 3: 实现 `abstract_multi_command_executor.rs`**

```rust
use super::command_executor::{CommandExecutor, CoreError};

pub type ExecutorFn<R> = Box<dyn Fn(R) -> R>;

pub struct AbstractMultiCommandExecutor<Ctx, Cmd, R> {
    pub inner: Box<dyn CommandExecutor<Ctx, Cmd, R>>,
    pub steps: Vec<ExecutorFn<R>>,
}

impl<Ctx, Cmd, R> AbstractMultiCommandExecutor<Ctx, Cmd, R>
where
    Cmd: super::command::Command,
{
    pub fn new(inner: Box<dyn CommandExecutor<Ctx, Cmd, R>>, steps: Vec<ExecutorFn<R>>) -> Self {
        Self { inner, steps }
    }
}

impl<Ctx, Cmd, R> CommandExecutor<Ctx, Cmd, R> for AbstractMultiCommandExecutor<Ctx, Cmd, R>
where
    Cmd: super::command::Command,
{
    fn execute(&self, ctx: &Ctx, cmd: Cmd) -> Result<R, CoreError> {
        let mut r = self.inner.execute(ctx, cmd)?;
        for step in &self.steps { r = step(r); }
        Ok(r)
    }
}
```

- [ ] **Step 4: 实现 `multi_command_executor.rs`**

```rust
use super::abstract_multi_command_executor::{AbstractMultiCommandExecutor, ExecutorFn};
use super::command::Command;
use super::command_executor::{CommandExecutor, CoreError};

pub type MultiCommandExecutor<Ctx, Cmd, R> =
    AbstractMultiCommandExecutor<Ctx, Cmd, R>;

impl<Ctx, Cmd, R> MultiCommandExecutor<Ctx, Cmd, R>
where
    Cmd: Command,
{
    pub fn new(inner: Box<dyn CommandExecutor<Ctx, Cmd, R>>, steps: Vec<ExecutorFn<R>>) -> Self {
        Self { inner, steps }
    }
}
```

- [ ] **Step 5: 运行测试，确认通过**

```bash
cargo test -p cqrs-4-rust-core --test multi_command_executor_test
```

- [ ] **Step 6: 提交**

```bash
git add crates/core/src/command_executor.rs crates/core/src/abstract_multi_command_executor.rs crates/core/src/multi_command_executor.rs crates/core/src/lib.rs crates/core/tests/multi_command_executor_test.rs
git commit -m "feat(core): add CommandExecutor + MultiCommandExecutor"
```

---

## Task 1.5: 实现 `CommandExecutionFailedError` / `UrlParamEntityIdPathNotEqualsCmdError`

**Files:**
- `crates/core/src/command_execution_failed_error.rs`
- `crates/core/src/url_param_entity_id_path_not_equals_cmd_error.rs`
- `crates/core/tests/command_execution_failed_error_test.rs`
- `crates/core/tests/url_param_entity_id_path_not_equals_cmd_error_test.rs`

- [ ] **Step 1: 实现 `CommandExecutionFailedError`**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[error("command execution failed: {message} (short_id={short_id})")]
pub struct CommandExecutionFailedError {
    pub short_id: String,
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CommandExecutionFailedError {
    pub fn new(short_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self { short_id: short_id.into(), message: message.into(), source: None }
    }
}
```

- [ ] **Step 2: 实现 `UrlParamEntityIdPathNotEqualsCmdError`**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[error("URL parameter entity id '{url_id}' does not equal command entity id '{cmd_id}'")]
pub struct UrlParamEntityIdPathNotEqualsCmdError {
    pub url_id: String,
    pub cmd_id: String,
}

impl UrlParamEntityIdPathNotEqualsCmdError {
    pub fn new(url_id: impl Into<String>, cmd_id: impl Into<String>) -> Self {
        Self { url_id: url_id.into(), cmd_id: cmd_id.into() }
    }
}
```

- [ ] **Step 3: 写测试覆盖字段、`source()`、`Display`**

- [ ] **Step 4: 提交**

```bash
git add crates/core/src/command_execution_failed_error.rs crates/core/src/url_param_entity_id_path_not_equals_cmd_error.rs crates/core/src/lib.rs crates/core/tests/command_execution_failed_error_test.rs crates/core/tests/url_param_entity_id_path_not_equals_cmd_error_test.rs
git commit -m "feat(core): add CommandExecutionFailedError + UrlParamEntityIdPathNotEqualsCmdError"
```

---

## Task 1.6: 实现 `CqrsUtils`（Adler-32 与 Java 等价）

**Files:**
- `crates/core/src/cqrs_utils.rs`
- `crates/core/tests/cqrs_utils_test.rs`

- [ ] **Step 1: 写失败测试**

```rust
use cqrs_4_rust_core::cqrs_utils::CqrsUtils;

#[test]
fn adler32_empty() {
    assert_eq!(CqrsUtils::adler32(&[]), 1);
}

#[test]
fn adler32_ascii_java_golden() {
    // Java 计算 "abc" Adler-32 = 0x024D0127
    let bytes = b"abc";
    assert_eq!(CqrsUtils::adler32(bytes), 0x024D0127u32);
}
```

- [ ] **Step 2: 实现 Adler-32**

```rust
pub struct CqrsUtils;
impl CqrsUtils {
    pub fn adler32(bytes: &[u8]) -> u32 {
        const MOD: u32 = 65521;
        let mut a: u32 = 1;
        let mut b: u32 = 0;
        for byte in bytes {
            let v = *byte as u32;  // ASCII 字节等价
            a = (a + v) % MOD;
            b = (b + a) % MOD;
        }
        (b << 16) | a
    }
}
```

- [ ] **Step 3: 运行测试，确认通过**

```bash
cargo test -p cqrs-4-rust-core --test cqrs_utils_test
```

- [ ] **Step 4: 提交**

```bash
git add crates/core/src/cqrs_utils.rs crates/core/tests/cqrs_utils_test.rs
git commit -m "feat(core): CqrsUtils.adler32 matches Java implementation"
```

---

## Task 1.7: 实现 `View` / `JpaView` / `JpaEventHandler`

**Files:**
- `crates/core/src/view.rs`
- `crates/core/src/jpa_view.rs`
- `crates/core/src/jpa_event_handler.rs`

- [ ] **Step 1: 实现三个 trait**

```rust
// view.rs
pub trait View {
    type E;
    fn name(&self) -> &str;
    fn cron(&self) -> &str;
    fn event_types(&self) -> &[&'static str];
    fn chunk_size(&self) -> usize { 100 }
    fn handle_events(&self, events: &[Self::E]);
}

// jpa_view.rs
use super::view::View;
pub trait JpaView: View {}

// jpa_event_handler.rs
use super::event::Event;
pub trait JpaEventHandler<E: Event> {
    fn event_type(&self) -> &'static str;
    fn handle(&self, event: &E);
}
```

- [ ] **Step 2: 写 `architecture_test.rs` 守门**

验证所有实现都满足 trait 形态约束。

- [ ] **Step 3: 提交**

```bash
git add crates/core/src/view.rs crates/core/src/jpa_view.rs crates/core/src/jpa_event_handler.rs crates/core/src/lib.rs crates/core/tests/architecture_test.rs
git commit -m "feat(core): add View/JpaView/JpaEventHandler traits"
```

---

## Task 1.8: 终态校验

- [ ] **Step 1: 全量门禁**

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test -p cqrs-4-rust-core --all-features
cargo clippy -p cqrs-4-rust-core --all-targets --all-features -- -D warnings
cargo doc -p cqrs-4-rust-core --no-deps
./tools/check_migration_parity.sh
```

期望：全部通过。

- [ ] **Step 2: 提交 Phase 1 完成标记**

```bash
git commit --allow-empty -m "chore(core): Phase 1 migration complete"
```

---

## 完成标志

- 21 个核心职责（15 main + 6 tests）齐全
- `cargo test -p cqrs-4-rust-core --all-features` 全通过
- Adler-32 与 Java 黄金值一致
- `tools/check_migration_parity.sh` 通过