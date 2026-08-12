# AGENTS.md — execution rules for any agentic worker in cqrs-4-rust

> Read this **before** touching the codebase. The superpowers layout
> (`docs/superpowers/{plans,specs}/`) is the single source of design +
> plan truth. The migration accounting CSV is the single source of file-
> mapping truth.

## 1. Knowledge hierarchy (read in order)

1. **Specs** (`docs/superpowers/specs/2026-07-23-*-design.md`) — design truth.
   - Read the matching spec **before** the matching plan.
2. **Plans** (`docs/superpowers/plans/2026-07-23-*-phase.md`,
   `2026-07-23-cqrs-4-rust-140-file-migration.md`) — TDD task lists with
   `[ ]` checkboxes. The plan you execute defines your `Done`.
3. **Migration accounting** (`docs/superpowers/plans/2026-07-23-cqrs-4-rust-migration-accounting.csv`)
   — 140-row Java ↔ Rust bidirectional file mapping. **Must not be edited
   outside the parity script.**

## 2. Coding rules (mirrored from `architecture-design` §6)

- Edition **2024**, resolver **3**, MSRV **1.88.0** (see `rust-toolchain.toml`).
- Modules: small = single `foo.rs`; submodules = `foo.rs + foo/`; never `mod.rs`.
- `lib.rs` does only module declarations + crate doc + targeted `pub use`. No
  glob re-export.
- Default private; public API uses `pub`, internal collaboration `pub(crate)`.
- Workspace dependencies declared once in root `[workspace.dependencies]`;
  members use `.workspace = true`.
- Tokio: never `features = ["full"]`. Only enable what is used.
- All public API errors must implement `std::error::Error` + `source()`.

## 3. Completion gates (must pass before opening PR)

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
./tools/check_migration_parity.sh
```

Forbidden: `todo!()`, `unimplemented!()`, empty dispatch bodies, logging-only
schedulers, placeholder helpers.

## 4. Process for picking up an issue

1. Find your issue via `gh issue view <N>` or browse
   `https://github.com/ddd-4-rust/cqrs-4-rust/issues?q=is%3Aopen+label%3A<your-area>`.
2. Read the **来源 plan** section in the issue body — open that plan file.
3. Work the TDD steps in order: failing test → run (fails) → implement → run
   (passes) → commit.
4. Update the issue body: tick the `- [ ]` boxes that you completed.
5. Commit with `git commit -m "<type>(<area>): <short>"`. Suggested types:
   `feat / fix / refactor / test / docs / chore`.
6. Open a PR. The CI `parity` job will block if you broke the 140-file
   baseline.

## 5. Process for the parity script

`./tools/check_migration_parity.sh` is the canonical authority for "did the
140-file mapping stay intact?". It is **fail-fast**:

- Java source files (excluding Maven Wrapper) must equal **140**.
- CSV mapping rows must equal **140**.
- Rust files referenced by `present` must all exist.

Do **not** add files outside the mapping without also extending the CSV.
Do **not** mark a Java source as `planned` without a corresponding Rust
file.

## 6. Working with the deferred KurrentDB / MariaDB stack

`MemoryProjectionAdmin` / `RegistryEventDecoder` / `MemoryEventStore` are
in-process substitutes for the real stack. The real KurrentDB gRPC
`ProjectionAdmin` client and MariaDB projection-position storage are
tracked in https://github.com/ddd-4-rust/cqrs-4-rust/issues/9 (Task 6 of the
top-level plan). Integration tests gate on `RUN_E2E=1`; see
`.github/workflows/e2e.yml`.

## 7. Branches

- `main` — released state. Fast-forward only from `dev`; CI must be green.
- `dev` — integration. PRs land here first.

## 8. When you are stuck

1. Re-read the matching spec (`docs/superpowers/specs/`).
2. Search the codebase via the `code-review-graph` MCP tools
   (`get_impact_radius`, `query_graph`, `get_flow`) before grepping.
3. If you discover a plan/spec gap, open a new issue with
   `type/spec` / `type/plan` and reference the parent plan.

Last updated: 2026-08-12 — initial AGENTS.md commit alongside CI workflow
bootstrap.