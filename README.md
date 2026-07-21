# cqrs-4-rust

> **Idiomatic Rust port of [`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)** —
> Command Query Responsibility Segregation building blocks for Rust.
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2021-orange)](https://doc.rust-lang.org/edition-guide/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v2-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.6.0-blue)](https://github.com/ddd-4-rust/cqrs-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/Port%20of-fuinorg/cqrs--4--java-green?logo=github)](https://github.com/fuinorg/cqrs-4-java)
[![Progress](https://img.shields.io/badge/Migration-88%25-brightgreen)](docs/MIGRATION_STATUS.md)

---

## 🙏 Acknowledgement — Java Source / 致谢

This project is a **one-to-one Rust translation** of:

> **[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)**
> by [Michael Schnell / fuinorg](https://github.com/fuinorg)
> *"Base classes for Command Query Responsibility Segregation (CQRS) with Java"*
>
> 📜 Original Java source license: **LGPLv3**
> 📌 Source version base: **0.6.0**

All architectural credit, API patterns, and CQRS insights come from the **`fuinorg`** project. This Rust port re-implements the same primitives with idiomatic Rust: traits for commands/queries, async handlers, and event store SPIs.

| | Java (source) | Rust (this port) |
|---|---|---|
| **Repository** | [fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java) | [ddd-4-rust/cqrs-4-rust](https://github.com/ddd-4-rust/cqrs-4-rust) |
| **Owner** | [fuinorg](https://github.com/fuinorg) | [ddd-4-rust org](https://github.com/ddd-4-rust) |
| **Maintainer** | Michael Schnell | hiwepy |
| **Language** | Java 17 | Rust 2021 |
| **License** | LGPLv3 | Apache 2.0 |
| **Version base** | 0.6.0 | 0.6.0 |
| **API compatibility** | — | 1:1 functional (idiomatic where required) |

---

## 🎯 What is this?

**`cqrs-4-rust`** provides the **CQRS building blocks** that complement [`ddd-4-rust`](../ddd-4-rust):

- **`Command<C>`** — marker trait for write-side operations
- **`CommandHandler<C, E>`** — async handler with associated error type
- **`CommandExecutor`** — dispatcher / router for commands
- **`Query<Q, R>`** — marker trait for read-side operations
- **`QueryHandler<Q, R>`** — async read handler
- **`View<A, V>`** — projection / read-model abstraction
- **`EventHandler<E>`** — domain event listener (sync / async variants)

Built around `async-trait` so the same traits work for any async runtime (tokio, async-std, smol).

---

## 🧱 Workspace Architecture / Workspace 结构

```text
cqrs-4-rust/                    ← Cargo Workspace (resolver = 2)
├── core/                       ← Command / CommandHandler / Query / View / EventHandler
│   └── cqrs-4-rust-core
├── serde/                      ← Serde adapters for cross-handler message serialization
│   └── cqrs-4-rust-serde
├── esc/                        ← Event Sourcing Context — projection rebuilding helpers
│   └── cqrs-4-rust-esc
├── actix/                      ← actix-web integration: HttpCommand/HttpQuery extractors
│   └── cqrs-4-rust-actix
├── axum/                       ← axum integration: JSON command/query dispatch
│   └── cqrs-4-rust-axum
├── test/                       ← 共享测试工具
│   └── cqrs-4-rust-test
└── docs/
    ├── ARCHITECTURE.md
    ├── IMPLEMENTATION_PLAN.md
    └── MIGRATION_STATUS.md
```

### Crate Map / Crate 一览

| Crate | Version | Responsibility | Key Dependencies |
|---|---|---|---|
| `cqrs-4-rust-core` | 0.6.0 | Command / Query / View / EventHandler traits | ddd-4-rust-core |
| `cqrs-4-rust-serde` | 0.6.0 | Serde adapters for cross-handler messages | core, ddd-4-rust |
| `cqrs-4-rust-esc` | 0.6.0 | Event Sourcing Context — projection rebuilding | core, ddd-4-rust-esc |
| `cqrs-4-rust-actix` | 0.6.0 | actix-web integration | core, esc |
| `cqrs-4-rust-axum` | 0.6.0 | axum integration | core, esc |
| `cqrs-4-rust-test` | 0.6.0 | Test utilities | core |

---

## 🚀 Quick Start

### Installation

```toml
[dependencies]
cqrs-4-rust-core = "0.6"
ddd-4-rust-core = "0.7"
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
```

> ⚠️ **Not yet published to crates.io.** Use git/path dependency until then:
>
> ```toml
> cqrs-4-rust-core = { git = "https://github.com/ddd-4-rust/cqrs-4-rust", branch = "main" }
> ```

### Define a Command + Handler

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

### Define a Query + Handler

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

### Wire to axum

```rust
use cqrs_4_rust_axum::dispatch;

let app = Router::new()
    .route("/orders/place", post(dispatch::<PlaceOrder, _>(exec.clone())))
    .route("/orders/:id", get(dispatch::<FindOrderById, _>(qexec.clone())));
```

### Build & Test

```bash
cargo build --workspace
cargo test  --workspace
cargo doc   --workspace --no-deps --open
```

---

## 🆚 Differences from the Java version / 与 Java 版本的差异

| Aspect | Java (fuinorg/cqrs-4-java) | Rust (this port) |
|---|---|---|
| Handler signature | `void / T` | `async fn` returning `Result<Vec<Event>, E>` |
| Serialization | Jackson modules | Serde with feature flags |
| Transport | HTTP / JMS agnostic | `actix` / `axum` adapter crates |
| Dispatch | Spring `@Component` | Manual `CommandExecutor` wiring or DI container |
| Async | `CompletableFuture` | `async-trait` + tokio |

---

## 📊 Migration Status / 迁移进度

> Last updated: 2026-07-21

| crate | Target .rs files | Completed | Completion |
|---|---|---|---|
| `cqrs-4-rust-core` | 13 | 12 | 92% |
| `cqrs-4-rust-serde` | 7 | 6 | 86% |
| `cqrs-4-rust-esc` | 3 | 3 | 100% |
| `cqrs-4-rust-actix` | 4 | 3 | 75% |
| `cqrs-4-rust-axum` | 4 | 3 | 75% |
| `cqrs-4-rust-test` | 1 | 1 | 100% |
| **Overall** | **~35** | **28** | **~88%** |

Full status: see [`docs/MIGRATION_STATUS.md`](docs/MIGRATION_STATUS.md).

---

## 📚 Related Projects / 相关项目

- 🏛️ **[ddd-4-rust](https://github.com/ddd-4-rust/ddd-4-rust)** — DDD primitives (required dependency)
- 🧪 **[ddd-cqrs-4-rust-example](https://github.com/ddd-4-rust/ddd-cqrs-4-rust-example)** — Full end-to-end example
- 🏛️ **[ddd-4-rust org](https://github.com/ddd-4-rust)** — Parent organization
- ☕ **[fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java)** — Java source (LGPLv3)
- 🧠 [DDD Mindmap](https://www.mindmeister.com/de/177813182/ddd) — DDD 概念地图
- 🧠 [CQRS Mindmap](https://www.mindmeister.com/de/177815383/cqrs) — CQRS 概念地图

---

## 📄 License

This Rust port is licensed under **Apache 2.0** — see [LICENSE](LICENSE).

The original Java source from [`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java) is licensed under **LGPLv3**. By the terms of LGPLv3, derivative works may use a different license, but the original must be credited. We do so prominently in the Acknowledgement section above.

---

## 🤝 Contributing / 贡献

Contributions are welcome! Before submitting a PR:

- [ ] Run `cargo fmt --all -- --check`
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] Run `cargo test --workspace`
- [ ] Add unit tests for new public APIs
- [ ] Update relevant docs (CHANGELOG, MIGRATION_STATUS)

---

<div align="center">

**Made with ❤️ by [ddd-4-rust](https://github.com/ddd-4-rust)**
**Ported from [fuinorg/cqrs-4-java](https://github.com/fuinorg/cqrs-4-java) by Michael Schnell**

</div>