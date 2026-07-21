# cqrs-4-rust 架构总览

## 分层架构

```text
┌─────────────────────────────────────────────────────────────┐
│  应用层（用户代码）                                            │
│    use cqrs_4_rust_core::prelude::*;                        │
│    struct CreatePersonCommand;                               │
└─────────────────────────┬───────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
┌───────▼───────┐  ┌──────▼──────┐  ┌──────▼──────────────┐
│  cqrs-4-rust- │  │  cqrs-4-rust- │  │  cqrs-4-rust-       │
│  core         │  │  serde       │  │  actix / axum        │
│  (同步)       │  │  (同步)      │  │  (async)             │
│               │  │              │  │                      │
│  Command      │  │  AbstractCmd │  │  ViewManager         │
│  CommandExec  │  │  CqrsResult  │  │  EventStoreConfig    │
│  Result       │  │  DataResult  │  │  ProjectionPosition  │
│  View         │  │  SimpleResult│  │                      │
│  JpaView      │  │              │  └──────────────────────┘
│  JpaEventHandler│  └──────────────┘
└───────────────┘
```

## 核心组件映射

| Java | Rust | crate |
|---|---|---|
| `Command extends Event` | `Command: Event` trait | core |
| `AggregateCommand<ROOT, ENTITY>` | `AggregateCommand<RootId, EntityIdType>` trait | core |
| `CommandExecutor<CTX, RESULT, CMD>` | `CommandExecutor<Ctx, Cmd>` trait | core |
| `Result<DATA>` | `CqrsResult<D>` struct | core |
| `SimpleResult` | `type SimpleResult = CqrsResult<()>` | serde |
| `DataResult<DATA>` | `type DataResult<D> = CqrsResult<D>` | serde |
| `JpaView` | `JpaView` trait | core |
| `JpaEventHandler<TYPE>` | `JpaEventHandler<E>` trait | core |
| `SpringJpaViewManager` | `ViewManager` (actix) | actix |
| `QuarkusJpaViewManager` | `ViewManager` (axum) | axum |
