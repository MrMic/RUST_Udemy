# CLAUDE.md — Rust Project

## Language & Toolchain

- **Rust edition**: 2024 (used in this workspace)
- **MSRV** (Minimum Supported Rust Version): declare explicitly in each member's `Cargo.toml` via `rust-version`
- Use `rustup` to manage toolchains; never hardcode a nightly version unless strictly necessary
- Formatter: `rustfmt` (run via `cargo fmt --all`) — must pass before any commit
- Linter: `clippy` (run via `cargo clippy --all -- -D warnings`) — treat all warnings as errors

## Workspace Structure

This project is a **Cargo workspace** with resolver `"3"`.

```
Cargo.toml          # workspace root — [workspace] with members list, shared deps
Cargo.lock          # single lock file for the entire workspace
members/            # all crates live here
  <crate-name>/
    Cargo.toml      # [package] only — inherit shared deps from workspace
    src/
      main.rs       # binary entry point — thin, delegates to lib
      lib.rs        # library root — all reusable logic lives here
      error.rs      # unified error type(s)
      config.rs     # configuration structs
      domain/       # core business logic, no I/O
      infra/        # I/O: DB, HTTP, filesystem
      cli/          # CLI argument parsing (clap)
    tests/          # integration tests (use real dependencies, no mocks of internals)
    benches/        # criterion benchmarks
    examples/       # runnable usage examples
```

### Adding a new crate

```bash
cargo new members/<crate-name>          # binary
cargo new members/<crate-name> --lib    # library
```

Then register it in the root `Cargo.toml`:

```toml
[workspace]
members = ["members/<crate-name>", ...]
```

### Workspace dependency sharing

- Declare shared dependencies once in the root `Cargo.toml` under `[workspace.dependencies]`
- Reference them in member crates with `dep = { workspace = true }`
- Override features per-member if needed: `dep = { workspace = true, features = ["extra"] }`

### Running workspace-wide commands

```bash
cargo build --all          # build all members
cargo test --all           # test all members
cargo fmt --all            # format all members
cargo clippy --all         # lint all members
cargo doc --no-deps --all  # doc all members
```

- Keep `main.rs` under 50 lines — it parses args and calls into `lib`
- One module = one concern; prefer many small modules over one large file
- Use `pub(crate)` instead of `pub` unless the item is part of the public API

## Error Handling

- Never use `.unwrap()` or `.expect()` in library code — propagate errors
- `.unwrap()` is acceptable only in tests and in `main` after a clear comment
- Define a unified `Error` enum per crate using `thiserror`
- Use `anyhow` only in binary crates (`main.rs`, CLI handlers) for ergonomic error reporting
- Use `?` for propagation — avoid manual `match` on `Result` unless you need to handle a specific variant
- Never silently swallow errors with `let _ = ...` unless intentional and commented

## Ownership & Borrowing

- Prefer borrowing (`&T`, `&mut T`) over cloning unless ownership transfer is semantically correct
- Avoid `clone()` in hot paths — profile first, then decide
- Use `Cow<str>` when a function may or may not need to own its string data
- Prefer `&str` over `String` in function parameters; return `String` when ownership is needed

## Types & Traits

- Model domain invariants in types — use newtypes (`struct UserId(u64)`) to prevent misuse
- Implement `Display` and `Debug` for all public types
- Derive `Clone`, `PartialEq`, `Eq` only when semantically appropriate — not by default
- Use `Default` only when a zero-value is meaningful for the type
- Avoid `Box<dyn Trait>` when a generic `impl Trait` works — prefer static dispatch
- Use `Box<dyn Error + Send + Sync>` as the trait object for errors only at API boundaries

## Async

- Runtime: `tokio` with the `#[tokio::main]` macro on `main`
- Use `async fn` only where I/O is involved — keep CPU-bound logic synchronous
- Spawn CPU-heavy work with `tokio::task::spawn_blocking`
- Avoid holding a `MutexGuard` across an `.await` — use `tokio::sync::Mutex` when needed
- Prefer `tokio::sync::RwLock` over `Mutex` for read-heavy shared state
- Cancel safety: document whether your async functions are cancel-safe

## Dependencies (`Cargo.toml`)

- Add a new dependency only when there is no reasonable stdlib alternative
- Prefer well-maintained crates with recent activity and a wide download base
- Pin major versions in `Cargo.toml` (e.g. `tokio = "1"`, not `tokio = "*"`)
- Run `cargo audit` before merging — fix or acknowledge all advisories
- Group features explicitly; avoid enabling default features blindly (`default-features = false`)
- Separate `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` cleanly

## Testing

- Unit tests: in the same file as the code, under `#[cfg(test)]`
- Integration tests: in `tests/` — these use the public API only
- Property-based testing: use `proptest` or `quickcheck` for data-invariant checks
- Snapshot testing: use `insta` for complex output comparisons
- Benchmarks: use `criterion` in `benches/` — never benchmark in unit tests
- Minimum coverage target: critical paths must have tests; aim for meaningful coverage, not 100%
- Use `#[should_panic]` only when panicking is the documented contract

## Performance

- Profile before optimizing — use `cargo flamegraph` or `perf`
- Avoid premature allocation: prefer iterators and lazy evaluation
- Use `Vec::with_capacity` when the size is known ahead of time
- Prefer `&[T]` over `&Vec<T>` in function signatures
- Use `SmallVec` or `arrayvec` for small collections that rarely exceed a fixed size
- Avoid `HashMap::new()` in hot paths — use `AHashMap` or pre-size with `with_capacity`

## Safety & Unsafe

- Do NOT write `unsafe` code without explicit user approval
- Every `unsafe` block must have a `// SAFETY:` comment explaining the invariant upheld
- Prefer safe abstractions (`slice::from_raw_parts` wrappers, etc.) over raw pointer arithmetic
- Run `cargo miri test` to catch undefined behavior in unsafe code

## Logging & Observability

- Use `tracing` (not `log`) for structured, async-aware logging
- Attach context with `tracing::instrument` on public async functions
- Use log levels correctly: `error` > `warn` > `info` > `debug` > `trace`
- Never log secrets, PII, or credentials — even at `trace` level

## Documentation

- All public items must have `///` doc comments
- Include a `# Examples` section in doc comments for non-trivial public functions
- Run `cargo doc --no-deps --document-private-items` to verify docs build without warnings
- Write `README.md` with: project purpose, quick start, cargo features table, and license

## CI Checklist (must pass before merge)

```bash
cargo fmt --all --check
cargo clippy --all -- -D warnings
cargo test --all
cargo doc --no-deps --all
cargo audit
```

Optional but recommended:

```bash
cargo deny check          # license + dependency policy
cargo mutants             # mutation testing
cargo miri test           # undefined behavior check (slow)
```

## Git Commit Convention

Follow Conventional Commits (same as global CLAUDE.md):

```
feat(parser): add support for nested expressions
fix(db): prevent connection leak on query timeout
perf(cache): replace HashMap with AHashMap in hot path
chore(deps): upgrade tokio to 1.37
```

- `feat` / `fix` / `perf` / `refactor` / `test` / `chore` / `docs`
- Subject ≤ 50 chars, imperative mood, no trailing period

## Do NOT

- Do not use `.unwrap()` in library or production code
- Do not `clone()` to satisfy the borrow checker — fix the ownership model instead
- Do not introduce `unsafe` without prior approval
- Do not add dependencies without checking for a stdlib or existing-crate alternative
- Do not merge code that fails `clippy -D warnings`
- Do not expose implementation details in the public API
- Do not use `String` where `&str` suffices in function parameters
