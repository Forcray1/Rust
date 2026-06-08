# 00 — Setup & Toolchain

## Install (you've likely done this)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version && cargo --version
```

You already have Xcode Command Line Tools, which provide the linker Rust needs. Good.

## The tools you'll use daily

| Tool | What it is | Python/C analogy |
|------|-----------|------------------|
| `rustc` | the compiler | `gcc` / there's no Python equivalent |
| `cargo` | build tool + package manager + test runner + more | `pip` + `make` + `venv` + `pytest`, all in one |
| `rustup` | toolchain manager (versions, components) | `pyenv` |
| `rust-analyzer` | the language server for your editor | `pylance` / `clangd` |

`cargo` is the center of your world. You rarely call `rustc` directly.

## The core cargo commands

```bash
cargo new myproject      # create a new project (a "crate")
cargo run                # compile + run (debug build)
cargo build              # compile only
cargo build --release    # optimized build (slow compile, fast binary)
cargo check              # type-check WITHOUT producing a binary — fast feedback loop
cargo test               # run all tests
cargo fmt                # auto-format your code
cargo clippy             # lint — catches bad patterns; install: rustup component add clippy
cargo add tokio          # add a dependency (edits Cargo.toml for you)
```

> `cargo check` is your fastest feedback loop. Use it constantly while learning —
> it tells you if the code is *valid* without the time of a full build.

## Anatomy of a project

```
myproject/
├── Cargo.toml        # manifest: name, version, dependencies (like pyproject.toml / package.json)
├── Cargo.lock        # exact dependency versions (auto-managed; commit it for apps)
└── src/
    └── main.rs       # entry point; must contain `fn main()`
```

`Cargo.toml` looks like:

```toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2021"          # the language edition — use 2021 (or newer if available)

[dependencies]
# crate_name = "version"  e.g.  tokio = { version = "1", features = ["full"] }
```

## Your first program

```bash
cargo new hello
cd hello
cargo run
```

`src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");   // println! is a MACRO (note the !), not a function
}
```

## Editor setup

Install **VS Code** (or RustRover, Zed, Neovim) and the **rust-analyzer** extension.
It gives you: inline type hints, autocomplete, jump-to-definition, and inline error squiggles
*before* you compile. This is not optional — Rust's type info is too valuable to code without it.

## Mental model shift from Python

- There's no REPL-first workflow. You compile. But compiling is fast with `cargo check`.
- The compiler is *strict on purpose*. In Python a bug shows up at runtime (maybe in production).
  In Rust the same bug usually won't compile. The pain moves earlier, where it's cheap.

## Mental model shift from C

- No header files. No manual `Makefile`. No `malloc`/`free`. No `-I`/`-l` flag juggling.
- Dependencies: you write one line in `Cargo.toml`, cargo fetches and builds them. No system
  package hunting.
- Memory is managed without a garbage collector *and* without manual `free` — that's module 02.

---

➡️ Next: `01_basics_vs_python_c.md`
