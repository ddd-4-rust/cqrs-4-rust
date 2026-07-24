# cqrs-4-rust

> **[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java) 的 idiomatic Rust 翻译版** —— Rust 的 CQRS（命令查询职责分离）基础构件库。
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-LGPL--3.0--or--later-blue.svg)](https://spdx.org/licenses/LGPL-3.0-or-later.html)
[![Rust](https://img.shields.io/badge/rust-1.88-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2024-orange)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v3-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.6.0-blue)](https://github.com/ddd-4-rust/cqrs-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/移植自-fuinorg/cqrs--4--java-green?logo=github)](https://github.com/fuinorg/cqrs-4-java)
[![Progress](https://img.shields.io/badge/职责映射-140%2F140-brightgreen)](docs/MIGRATION_STATUS.md)

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
| **语言** | Java 17 | Rust 2024 Edition |
| **许可证** | LGPLv3 | LGPL-3.0-or-later |
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
cqrs-4-rust/                    ← Virtual Cargo workspace（resolver = 3）
├── crates/
│   ├── cqrs/                   ← feature-gated 公共 facade
│   ├── core/                   ← 与运行时无关的 CQRS 契约
│   ├── esc/                    ← Event Store Commons
│   ├── serialization/
│   │   ├── serde/              ← Java JSON 线协议的 Serde 实现
│   │   ├── jaxb/               ← JAXB 兼容 XML 线协议
│   │   └── jsonb/              ← JSON-B 线协议与 inventory 注册表
│   ├── adapter/
│   │   ├── actix/              ← Actix Web 适配器
│   │   └── axum/               ← Axum 适配器
│   └── test/
│       ├── support/            ← 共享集成测试支持
│       ├── actix/              ← Quarkus 来源的 Actix 集成模型
│       └── axum/               ← Spring 来源的 Axum 集成模型
└── docs/
    ├── ARCHITECTURE.md
    ├── IMPLEMENTATION_PLAN.md
    └── MIGRATION_STATUS.md
```

### Crate 一览

| Crate | 版本 | 职责 | 关键依赖 |
|---|---|---|---|
| `cqrs-4-rust` | 0.6.0 | feature-gated 公共 facade | core；可选适配器 |
| `cqrs-4-rust-core` | 0.6.0 | Command / Query / View / EventHandler trait | ddd-4-rust-core |
| `cqrs-4-rust-serde` | 0.6.0 | Java JSON 线协议的 Serde 实现 | core, ddd-4-rust-serde |
| `cqrs-4-rust-jaxb` | 0.6.0 | JAXB 兼容 XML 序列化 | core, ddd-4-rust-serde, quick-xml |
| `cqrs-4-rust-jsonb` | 0.6.0 | JSON-B 兼容 Serde adapter 与编译期注册表 | core, serde_json, inventory |
| `cqrs-4-rust-esc` | 0.6.0 | 事件溯源上下文 — 投影重建 | core, ddd-4-rust-esc |
| `cqrs-4-rust-actix` | 0.6.0 | actix-web 集成 | core, esc |
| `cqrs-4-rust-axum` | 0.6.0 | axum 集成 | core, esc |
| `cqrs-4-rust-test-support` | 0.6.0 | 共享集成测试支持 | core |
| `cqrs-4-rust-test-actix` | 0.6.0 | Quarkus 来源的 Actix 集成模型 | actix-web, actix adapter |
| `cqrs-4-rust-test-axum` | 0.6.0 | Spring 来源的 Axum 集成模型 | axum, axum adapter |

---

## 🚀 快速开始

### 安装

```toml
[dependencies]
cqrs-4-rust = { version = "0.6", features = ["serde", "axum"] }
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
| 序列化 | Jackson 与 JSON-B 模块 | 带 feature 的 Serde 线协议 adapter |
| Spring Boot 集成 | Spring 调度与应用生命周期 | `axum` 适配 crate |
| Quarkus 集成 | Quarkus 调度与启动/关闭生命周期 | `actix` 适配 crate |
| 传输层 | HTTP / JMS 无关 | `axum` / `actix` 适配 crate |
| 分发 | Spring `@Component` | 手动 `CommandExecutor` 装配或 DI 容器 |
| 异步 | `CompletableFuture` | `async-trait` + tokio |

---

## 📊 迁移进度

> 2026-07-23 按 `cqrs-4-java` 0.6.0 重新审计。“已存在”仅表示 Rust 文件已经建立；
> 语义验收仍须通过对应的一比一测试。

| 范围 | Java 目标 | Rust 已存在 | 已验收 |
|---|---:|---:|---:|
| 已迁移的生产、生成、测试及覆盖率职责 | 140 | 140 | 0 |

文件数量对齐已经完成；真实数据库投影调度与 Docker/KurrentDB/MariaDB
端到端链路仍待语义验收。Maven Wrapper 引导类属于构建工具，是 141 个
Java 文件中唯一不进入迁移账本的文件。
目标 Rust workspace 和逐模块映射见
[`docs/IMPLEMENTATION_PLAN.md`](docs/IMPLEMENTATION_PLAN.md)。

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

本 Rust 移植版保留源项目的 **LGPL-3.0-or-later** 许可证，并在 workspace
Cargo manifest 中声明。

原 Java 源（来自
[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)）采用 LGPLv3，
并已在上面的“致谢”部分明确标注来源。

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
