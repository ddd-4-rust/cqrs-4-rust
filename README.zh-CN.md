# cqrs-4-rust

> **[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java) 的 idiomatic Rust 翻译版** —— Rust 的 CQRS（命令查询职责分离）基础构件库。
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2021-orange)](https://doc.rust-lang.org/edition-guide/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v2-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.6.0-blue)](https://github.com/ddd-4-rust/cqrs-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/移植自-fuinorg/cqrs--4--java-green?logo=github)](https://github.com/fuinorg/cqrs-4-java)
[![Progress](https://img.shields.io/badge/迁移进度-88%25-brightgreen)](docs/MIGRATION_STATUS.md)

---

## 🙏 致谢 — Java 源项目

本项目是以下项目的**一比一 Rust 翻译**：

> **[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)**
> 原作者：[Michael Schnell / fuinorg](https://github.com/fuinorg)
> *"Base classes for Command Query Responsibility Segregation (CQRS) with Java"*
>
> 📜 原 Java 源码许可证：**LGPLv3**
> 📌 源项目版本基线：**0.6.0**

**原项目的架构设计、API 形态和 CQRS 洞见**全部归功于 **`fuinorg`** 项目。本 Rust 移植版采用 idiomatic Rust 重新实现了同一套原语：commands/queries trait、async handler 和 event store SPI。

| | Java（源） | Rust（本移植版） |
|---|---|---|
| **仓库** | [fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java) | [ddd-4-rust/cqrs-4-rust](https://github.com/ddd-4-rust/cqrs-4-rust) |
| **所有者** | [fuinorg](https://github.com/fuinorg) | [ddd-4-rust 组织](https://github.com/ddd-4-rust) |
| **维护者** | Michael Schnell | hiwepy |
| **语言** | Java 17 | Rust 2021 |
| **许可证** | LGPLv3 | Apache 2.0 |
| **版本基线** | 0.6.0 | 0.6.0 |
| **API 兼容性** | — | 1:1 功能等价（必要时采用 idiomatic 写法） |

---

## 🎯 项目定位

**`cqrs-4-rust`** 提供 **`ddd-4-rust` 的配套 CQRS 基础构件**：

- **`Command<C>`** — 写操作标记 trait
- **`CommandHandler<C, E>`** — 带关联错误类型的 async handler
- **`CommandExecutor`** — command 分发/路由器
- **`Query<Q, R>`** — 读操作标记 trait
- **`QueryHandler<Q, R>`** — async 读 handler
- **`View<A, V>`** — 投影 / 读模型抽象
- **`EventHandler<E>`** — 领域事件监听器（同步 / 异步变体）

基于 `async-trait` 构建，同一套 trait 可用于任何 async runtime（tokio、async-std、smol）。

---

## 🧱 Workspace 结构

```text
cqrs-4-rust/                    ← Cargo Workspace (resolver = 2)
├── core/                       ← Command / CommandHandler / Query / View / EventHandler
│   └── cqrs-4-rust-core
├── serde/                      ← 跨 handler 消息序列化的 Serde 适配器
│   └── cqrs-4-rust-serde
├── esc/                        ← 事件溯源上下文 — 投影重建辅助
│   └── cqrs-4-rust-esc
├── actix/                      ← actix-web 集成：HttpCommand/HttpQuery extractors
│   └── cqrs-4-rust-actix
├── axum/                       ← axum 集成：JSON command/query 分发
│   └── cqrs-4-rust-axum
├── test/                       ← 共享测试工具
│   └── cqrs-4-rust-test
└── docs/
    ├── ARCHITECTURE.md
    ├── IMPLEMENTATION_PLAN.md
    └── MIGRATION_STATUS.md
```

### Crate 一览

| Crate | 版本 | 职责 | 关键依赖 |
|---|---|---|---|
| `cqrs-4-rust-core` | 0.6.0 | Command / Query / View / EventHandler trait | ddd-4-rust-core |
| `cqrs-4-rust-serde` | 0.6.0 | 跨 handler 消息的 Serde 适配器 | core, ddd-4-rust |
| `cqrs-4-rust-esc` | 0.6.0 | 事件溯源上下文 — 投影重建 | core, ddd-4-rust-esc |
| `cqrs-4-rust-actix` | 0.6.0 | actix-web 集成 | core, esc |
| `cqrs-4-rust-axum` | 0.6.0 | axum 集成 | core, esc |
| `cqrs-4-rust-test` | 0.6.0 | 测试工具 | core |

---

## 🚀 快速开始

### 安装

```toml
[dependencies]
cqrs-4-rust-core = "0.6"
ddd-4-rust-core = "0.7"
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
```

> ⚠️ **尚未发布到 crates.io。** 在发布之前，使用 git/path 依赖：
>
> ```toml
> cqrs-4-rust-core = { git = "https://github.com/ddd-4-rust/cqrs-4-rust", branch = "main" }
> ```

### 定义 Command + Handler

```rust
use cqrs_4_rust_core::prelude::*;
use ddd_4_rust_core::prelude::*;

#[derive(Debug, Clone, Command)]
pub struct PlaceOrder {
    pub id: OrderId,
    pub total: Money,
}

#[async_trait]
impl CommandHandler<PlaceOrder> for OrderAggregate {
    type Error = OrderError;

    async fn handle(&self, cmd: PlaceOrder) -> Result<Vec<OrderEvent>, Self::Error> {
        Ok(vec![OrderEvent::Created {
            id: cmd.id,
            total: cmd.total,
        }])
    }
}
```

### 定义 Query + Handler

```rust
#[derive(Debug, Clone, Query)]
pub struct FindOrderById {
    pub id: OrderId,
}

#[async_trait]
impl QueryHandler<FindOrderById, Option<OrderView>> for OrderReadModel {
    async fn handle(&self, q: FindOrderById) -> Result<Option<OrderView>, Self::Error> {
        self.repo.find(&q.id).await
    }
}
```

### 接入 axum

```rust
use cqrs_4_rust_axum::dispatch;

let app = Router::new()
    .route("/orders/place", post(dispatch::<PlaceOrder, _>(exec.clone())))
    .route("/orders/:id", get(dispatch::<FindOrderById, _>(qexec.clone())));
```

### 构建与测试

```bash
cargo build --workspace
cargo test  --workspace
cargo doc   --workspace --no-deps --open
```

---

## 🆚 与 Java 版本的差异

| 维度 | Java (fuinorg/cqrs-4-java) | Rust (本移植版) |
|---|---|---|
| Handler 签名 | `void / T` | `async fn` 返回 `Result<Vec<Event>, E>` |
| 序列化 | Jackson 模块 | Serde feature flags |
| 传输层 | HTTP / JMS 无关 | `actix` / `axum` 适配 crate |
| 分发 | Spring `@Component` | 手动 `CommandExecutor` 装配或 DI 容器 |
| 异步 | `CompletableFuture` | `async-trait` + tokio |

---

## 📊 迁移进度

> 最后更新：2026-07-21

| crate | 目标 .rs 文件 | 已完成 | 完成率 |
|---|---|---|---|
| `cqrs-4-rust-core` | 13 | 12 | 92% |
| `cqrs-4-rust-serde` | 7 | 6 | 86% |
| `cqrs-4-rust-esc` | 3 | 3 | 100% |
| `cqrs-4-rust-actix` | 4 | 3 | 75% |
| `cqrs-4-rust-axum` | 4 | 3 | 75% |
| `cqrs-4-rust-test` | 1 | 1 | 100% |
| **总计** | **~35** | **28** | **~88%** |

完整进度：[`docs/MIGRATION_STATUS.md`](docs/MIGRATION_STATUS.md)

---

## 📚 相关项目

- 🏛️ **[ddd-4-rust](https://github.com/ddd-4-rust/ddd-4-rust)** — DDD 原语（必需依赖）
- 🧪 **[ddd-cqrs-4-rust-example](https://github.com/ddd-4-rust/ddd-cqrs-4-rust-example)** — 端到端完整示例
- 🏛️ **[ddd-4-rust 组织](https://github.com/ddd-4-rust)** — 父组织
- ☕ **[fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java)** — Java 源项目（LGPLv3）
- 🧠 [DDD 概念地图](https://www.mindmeister.com/de/177813182/ddd)
- 🧠 [CQRS 概念地图](https://www.mindmeister.com/de/177815383/cqrs)

---

## 📄 许可证

本 Rust 移植版采用 **Apache 2.0** 许可证 —— 见 [LICENSE](LICENSE)。

原 Java 源（来自 [`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)）采用 **LGPLv3**。根据 LGPLv3 的条款，衍生作品可以使用不同的许可证，但必须明确标注原始来源。我们在上面的"致谢"部分显著地做了这一点。

---

## 🤝 贡献

欢迎贡献！提交 PR 前请确保：

- [ ] 运行 `cargo fmt --all -- --check`
- [ ] 运行 `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] 运行 `cargo test --workspace`
- [ ] 为新公共 API 添加单元测试
- [ ] 更新相关文档（CHANGELOG、MIGRATION_STATUS）

---

<div align="center">

**由 [ddd-4-rust](https://github.com/ddd-4-rust) 用 ❤️ 制作**
**移植自 Michael Schnell 的 [fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java)**

</div>