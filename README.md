# cqrs-4-rust

> **Idiomatic Rust port of [`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java)** —
> Command Query Responsibility Segregation building blocks for Rust.
>
> [English](README.md) | [简体中文](README.zh-CN.md)

[![License](https://img.shields.io/badge/License-LGPL--3.0--or--later-blue.svg)](https://spdx.org/licenses/LGPL-3.0-or-later.html)
[![Rust](https://img.shields.io/badge/rust-1.88-orange?logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2024-orange)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Workspace Resolver](https://img.shields.io/badge/Resolver-v3-blueviolet)](https://doc.rust-lang.org/cargo/reference/resolver.html)
[![Workspace Version](https://img.shields.io/badge/version-0.6.0-blue)](https://github.com/ddd-4-rust/cqrs-4-rust)
[![Org](https://img.shields.io/badge/Org-ddd--4--rust-6366f1)](https://github.com/ddd-4-rust)
[![Java Source](https://img.shields.io/badge/Port%20of-fuinorg/cqrs--4--java-green?logo=github)](https://github.com/fuinorg/cqrs-4-java)
[![Progress](https://img.shields.io/badge/Mapped%20responsibilities-140%2F140-brightgreen)](docs/MIGRATION_STATUS.md)

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
| **Language** | Java 17 | Rust 2024 Edition |
| **License** | LGPLv3 | LGPL-3.0-or-later |
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
cqrs-4-rust/                    ← Virtual Cargo workspace (resolver = 3)
├── crates/
│   ├── cqrs/                   ← Feature-gated public facade
│   ├── core/                   ← Runtime-independent CQRS contracts
│   ├── esc/                    ← Event-store commons
│   ├── serialization/
│   │   ├── serde/              ← Serde implementation of the Java JSON wire format
│   │   ├── jaxb/               ← JAXB-compatible XML wire format
│   │   └── jsonb/              ← JSON-B wire format and inventory registry
│   ├── adapter/
│   │   ├── actix/              ← Actix Web adapter
│   │   └── axum/               ← Axum adapter
│   └── test/
│       ├── support/            ← Shared integration-test support
│       ├── actix/              ← Quarkus-source Actix integration model
│       └── axum/               ← Spring-source Axum integration model
└── docs/
    ├── ARCHITECTURE.md
    ├── IMPLEMENTATION_PLAN.md
    └── MIGRATION_STATUS.md
```

### Crate Map / Crate 一览

| Crate | Version | Responsibility | Key Dependencies |
|---|---|---|---|
| `cqrs-4-rust` | 0.6.0 | Feature-gated public facade | core; optional adapters |
| `cqrs-4-rust-core` | 0.6.0 | Command / Query / View / EventHandler traits | ddd-4-rust-core |
| `cqrs-4-rust-serde` | 0.6.0 | Serde implementation of the Java JSON wire format | core, ddd-4-rust-serde |
| `cqrs-4-rust-jaxb` | 0.6.0 | JAXB-compatible XML serialization | core, ddd-4-rust-serde, quick-xml |
| `cqrs-4-rust-jsonb` | 0.6.0 | JSON-B-compatible Serde adapter and compile-time registry | core, serde_json, inventory |
| `cqrs-4-rust-esc` | 0.6.0 | Event Sourcing Context — projection rebuilding | core, ddd-4-rust-esc |
| `cqrs-4-rust-actix` | 0.6.0 | actix-web integration | core, esc |
| `cqrs-4-rust-axum` | 0.6.0 | axum integration | core, esc |
| `cqrs-4-rust-test-support` | 0.6.0 | Shared integration-test support | core |
| `cqrs-4-rust-test-actix` | 0.6.0 | Quarkus-source Actix integration model | actix-web, actix adapter |
| `cqrs-4-rust-test-axum` | 0.6.0 | Spring-source Axum integration model | axum, axum adapter |

---

## 🚀 Quick Start

### Installation

```toml
[dependencies]
cqrs-4-rust = { version = "0.6", features = ["serde", "axum"] }
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
| Serialization | Jackson and JSON-B modules | Serde wire adapters with feature flags |
| Spring Boot integration | Spring scheduler and application lifecycle | `axum` adapter crate |
| Quarkus integration | Quarkus scheduler and startup/shutdown lifecycle | `actix` adapter crate |
| Transport | HTTP / JMS agnostic | `axum` / `actix` adapter crates |
| Dispatch | Spring `@Component` | Manual `CommandExecutor` wiring or DI container |
| Async | `CompletableFuture` | `async-trait` + tokio |

---

## 📊 Migration Status / 迁移进度

> Audited against `cqrs-4-java` 0.6.0 on 2026-07-23. “Present” only means the
> Rust file exists; semantic acceptance still requires its parity tests to pass.

| Scope | Java target | Rust present | Accepted |
|---|---:|---:|---:|
| Migrated production, generated, test, and coverage responsibilities | 140 | 140 | 0 |

File-count parity is complete. Semantic acceptance remains open for the real
database-backed projection scheduler and Docker/KurrentDB/MariaDB end-to-end
path. The Maven Wrapper bootstrap class is tooling and is the only Java file excluded
from the 140-file migration ledger. The target Rust workspace and per-module
mapping are defined in [`docs/IMPLEMENTATION_PLAN.md`](docs/IMPLEMENTATION_PLAN.md).

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

This Rust port retains the source project's **LGPL-3.0-or-later** license, as
declared in the workspace Cargo manifest.

The original Java source from
[`fuinorg/cqrs-4-java`](https://github.com/fuinorg/cqrs-4-java) is licensed
under LGPLv3 and is credited in the Acknowledgement section above.

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
