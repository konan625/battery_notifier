# How to run Battery Notifier

This document explains how to build and run the `battery_notifier` Rust project locally.

Prerequisites
- Rust and Cargo installed (via `rustup`).

Quick steps (recommended)
1. Install or switch to the nightly toolchain (required if the project uses edition 2024):
```bash
rustup toolchain install nightly
rustup override set nightly
```
2. Build the project:
```bash
cd battery_notifier
cargo build
```
3. Run the project (debug build):
```bash
cargo run
```
4. Run the release build:
```bash
cargo run --release
```

Alternative: opt-in for edition 2024 in `Cargo.toml`
- If you prefer not to use nightly, add the following as the very first line of `Cargo.toml`:
```toml
cargo-features = ["edition2024"]
```
Note: this may still require a newer Cargo/nightly depending on your environment.

Alternative: change edition to 2021
- If the codebase doesn't require 2024-specific features, you can change `edition = "2024"` in `Cargo.toml` to `edition = "2021"` and then run `cargo build`.

Troubleshooting
- If `cargo build` fails with `feature "edition2024" is required`, use the nightly toolchain (first option) or add `cargo-features` to `Cargo.toml` (second option).
- If you see missing system notification libraries on Linux, ensure your desktop environment supports libnotify and that `notify-rust` dependencies are available.

Example: run binary directly
```bash
cd battery_notifier
cargo build
./target/debug/battery_notifier
```

If you want, I can also open a small PR that adds this file and a link in the project `README.md`.
