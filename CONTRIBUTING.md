# Contributing to NATP

For a bug report, include the theorem or Rust snippet, the result you expected,
and the output you received. Open an issue before making substantial changes to
the theorem language or proof rules.

## Development

Install Rust through rustup. Cargo uses the version pinned in `rust-toolchain.toml`.

```sh
cargo build --workspace --locked
cargo natp --help
```

Before opening a pull request, run:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --doc --locked
cargo build --workspace --release --locked
cargo doc --workspace --no-deps --locked
```

To check the minimum supported compiler:

```sh
rustup toolchain install 1.85.0 --profile minimal
cargo +1.85.0 check --workspace --all-targets --locked
```

## Pull requests

Keep each pull request focused on one change. Explain the problem, the resulting
behaviour and how you checked it. Include a small theorem or Rust snippet when it
helps demonstrate the change.

Keep certificate checking independent of proof search. Reserve `Proven` for
kernel proofs; bounded verification returns `Verified`, and external solver
validation returns `SmtValidated`.

Call out changes to theorem or certificate formats, including how existing files
are affected.
