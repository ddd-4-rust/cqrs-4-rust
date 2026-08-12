# cqrs-4-rust core 设计

- 日期: 2026-07-23
- 范围: `crates/core/` (21 个迁移职责)
- 状态: 已实现，待 JPA context 与异常语义复核

## 1. 背景

Java 版 `cqrs-4-java/core` 定义 CQRS 的最小契约：`Command`、`AggregateCommand`、`CommandExecutor`、`Result`、`ResultType`、`ToResultCapable`、两个领域异常 (`CommandExecutionFailedException`、`UrlParamEntityIdPathNotEqualsCmdException`)、`View` / `JpaView` / `JpaEventHandler` 三个接口、`CqrsUtils` 工具类、以及 `package-info.java` 占位。

Java 早期把 `AbstractMultiCommandExecutor` 与 `MultiCommandExecutor` 合在一个文件；迁移必须拆为独立文件，保持职责一一对应。

## 2. 目标

1. 还原 `CommandExecutor<Context, Result, Command>` 的结果泛型语义。
2. 补齐 `ToResultCapable` 转换能力。
3. 把两个 Java exception 映射为独立、结构化的 Rust error 文件。
4. `CqrsUtils` 中 Adler-32 计算保持 Java 的输入顺序与 ASCII 字节语义。
5. 6 个 Java 测试文件对应 6 个 Rust 测试文件。

## 3. 非目标

不引入异步执行器（属于 adapter 层职责）；不引入 Actor / Channel（破坏同步语义）。

## 4. 模块布局

```
crates/core/
├── src/
│   ├── lib.rs                       # package-info.java 对应
│   ├── abstract_multi_command_executor.rs
│   ├── multi_command_executor.rs
│   ├── aggregate_command.rs
│   ├── command.rs
│   ├── command_executor.rs
│   ├── command_execution_failed_error.rs   # 原 CommandExecutionFailedException
│   ├── cqrs_utils.rs
│   ├── jpa_event_handler.rs
│   ├── jpa_view.rs
│   ├── result.rs
│   ├── result_type.rs
│   ├── to_result_capable.rs
│   ├── url_param_entity_id_path_not_equals_cmd_error.rs  # 原 UrlParamEntityIdPathNotEqualsCmdException
│   └── view.rs
└── tests/
    ├── architecture_test.rs
    ├── base_test.rs
    ├── command_execution_failed_error_test.rs
    ├── cqrs_utils_test.rs
    ├── multi_command_executor_test.rs
    └── url_param_entity_id_path_not_equals_cmd_error_test.rs
```

## 5. 关键抽象

### 5.1 Command / AggregateCommand

```rust
pub trait Event {
    fn event_type(&self) -> &'static str;
}

pub trait Command: Event {}

pub trait AggregateCommand<RootId, EntityIdType> {
    fn aggregate_root_id(&self) -> &RootId;
    fn aggregate_entity_id(&self) -> Option<&EntityIdType>;
}
```

### 5.2 CommandExecutor（恢复结果泛型）

```rust
pub trait CommandExecutor<Ctx, Cmd: Command, R> {
    fn execute(&self, ctx: &Ctx, cmd: Cmd) -> Result<R, CoreError>;
}
```

注：Java 的 `Result<RESULT>` 由 Rust 的 `R` 泛型参数承担；`CqrsResult` 在 `core` 中只是占位，序列化由 adapter 提供。

### 5.3 CqrsResult / ResultType / ToResultCapable

```rust
pub enum ResultType { Ok, Warning, Error }

pub struct CqrsResult<D> {
    pub result_type: ResultType,
    pub data: Option<D>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

pub trait ToResultCapable {
    type Data;
    fn to_result(self) -> CqrsResult<Self::Data>;
}
```

### 5.4 View / JpaView / JpaEventHandler

```rust
pub trait View {
    type E: Event;
    fn name(&self) -> &str;
    fn cron(&self) -> &str;
    fn event_types(&self) -> &[&'static str];
    fn chunk_size(&self) -> usize { 100 }
    fn handle_events(&self, events: &[Self::E]);
}

pub trait JpaView: View {}

pub trait JpaEventHandler<E: Event> {
    fn event_type(&self) -> &'static str;
    fn handle(&self, event: &E);
}
```

## 6. 错误模型

每个 Java exception 对应一个独立 Rust 错误结构体，**不允许**合并到一个泛型 `enum`：

- `command_execution_failed_error.rs`：`CommandExecutionFailedError` 结构体携带原始 `cause`。
- `url_param_entity_id_path_not_equals_cmd_error.rs`：`UrlParamEntityIdPathNotEqualsCmdError`。

错误字符串与字段命名必须与 Java 等价（`ShortId` / `PersonId` 等）。

## 7. 测试覆盖

| 测试文件 | 覆盖 |
|---|---|
| `multi_command_executor_test.rs` | 多命令串联执行、错误传播、上下文传递 |
| `cqrs_utils_test.rs` | Adler-32 与 Java 等价（输入顺序 + ASCII 字节） |
| `command_execution_failed_error_test.rs` | cause 链、`short_id` / `message` 字段 |
| `url_param_entity_id_path_not_equals_cmd_error_test.rs` | path 参数与命令实体 ID 不一致错误 |
| `architecture_test.rs` | trait 实现合规（架构守门人） |
| `base_test.rs` | 测试 fixture 复用 |

## 8. 完成定义

- 21 个 `.rs` 文件齐全（21 main + 6 tests 合并 27 行 file_mapping；本规范以 21 职责为口径）
- `cargo test -p cqrs-4-rust-core --all-features` 通过
- `cargo clippy -p cqrs-4-rust-core --all-targets --all-features -- -D warnings` 通过
- Adler-32 实现以 Java 黄金值回归

## 9. 与 Java 的差异

| 差异 | 解释 |
|---|---|
| 异常结构体不带 stack trace | Rust error 携带 `source()`；Java `Throwable.printStackTrace` 不直接移植，由 logger::error! 替代 |
| `package-info.java` → `lib.rs` | 仅承担模块声明与 crate 文档 |
| `AbstractMultiCommandExecutor` 拆为独立文件 | 严格遵循 Rust 一个文件一个职责 |