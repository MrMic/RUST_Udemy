# Justfile — section03 workspace

default: check

# ── Workspace ─────────────────────────────────────────────────────────────────

check:
    cargo check --workspace

build:
    cargo build --workspace

build-release:
    cargo build --workspace --release

test:
    cargo test --workspace

clippy:
    cargo clippy --workspace -- -D warnings

# ── Bacon (continuous) ────────────────────────────────────────────────────────

bacon:
    bacon

bacon-run:
    bacon run

bacon-run-borrowing:
    bacon run -- -p borrowing

bacon-run-dereferencing:
    bacon run -- -p dereferencing

bacon-run-ownership:
    bacon run -- -p ownership

bacon-run-quick-startup:
    bacon run -- -p quick_startup

bacon-run-structs-basics:
    bacon run -- -p structs-basics

bacon-test:
    bacon test

bacon-clippy:
    bacon clippy

# ── Members ───────────────────────────────────────────────────────────────────

run member:
    cargo run -p {{member}}

check-member member:
    cargo check -p {{member}}

test-member member:
    cargo test -p {{member}}
