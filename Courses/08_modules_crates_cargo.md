# 08 — Modules, Crates & Cargo

How to grow from one `main.rs` file into a real, multi-file game project, and how to pull in the
ecosystem of libraries you'll need (networking, serialization, async).

## Vocabulary

- **Crate** = a compilation unit / package. A binary crate (has `main`) or a library crate (`lib`).
- **Module** (`mod`) = a namespace *within* a crate. Like Python modules/packages, but you declare
  the tree explicitly.
- **Cargo** = builds crates and fetches dependencies (other people's crates from crates.io).

## Modules within one crate

```rust
// src/main.rs
mod combat;          // declares: "there's a module `combat` (in src/combat.rs)"
mod world;

use combat::attack;  // bring a name into scope

fn main() {
    let dmg = attack(10, 5);
    println!("{dmg}");
}
```

```rust
// src/combat.rs
pub fn attack(power: u32, defense: u32) -> u32 {   // `pub` = visible outside this module
    power.saturating_sub(defense)
}

fn internal_helper() {}   // no `pub` → private to this module
```

### Visibility

Everything is **private by default** (unlike Python where everything's public). Use `pub` to
export. This is great for big codebases: the compiler enforces your API boundaries.

```rust
pub struct Player {
    pub name: String,    // public field
    hp: u32,             // private field — only this module can touch it
}
```

### File layout for a real project

```
src/
├── main.rs              // mod declarations + main()
├── world.rs            // mod world
├── combat.rs           // mod combat
├── net/                // a module with submodules:
│   ├── mod.rs           //   (or net.rs in 2021 edition) — declares submodules
│   ├── server.rs        //   net::server
│   └── protocol.rs      //   net::protocol
└── entity/
    ├── mod.rs
    ├── player.rs
    └── monster.rs
```

```rust
// src/net/mod.rs  (or src/net.rs)
pub mod server;
pub mod protocol;
```

Refer to things by path: `crate::net::protocol::ClientMessage`, or `use` them. `crate::` is the
root, `super::` is the parent module, `self::` is the current one.

## Splitting into a lib + bin (recommended for the MMORPG)

A common, clean structure: put your game logic in a **library crate** and have thin **binaries**
for the server and client:

```
mygame/
├── Cargo.toml
└── src/
    ├── lib.rs           // the reusable game library (world, entities, protocol)
    └── bin/
        ├── server.rs    // cargo run --bin server
        └── client.rs    // cargo run --bin client
```

```toml
# Cargo.toml
[package]
name = "mygame"
version = "0.1.0"
edition = "2021"
# src/lib.rs is the library; src/bin/*.rs are auto-detected binaries
```

Both binaries `use mygame::...` to share the same world/protocol code. This avoids duplicating
logic between client and server.

## Dependencies (crates.io)

Add a dependency:

```bash
cargo add serde --features derive
cargo add tokio --features full
cargo add anyhow
```

…which writes into `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

Then `use` them in code. Cargo downloads, version-resolves, and builds everything. No system
package manager, no `-l` flags, no header paths. `Cargo.lock` pins exact versions for reproducible
builds.

### Crates you'll likely use for an MMORPG

| Need | Crate |
|------|-------|
| Async runtime + networking | `tokio` |
| Serialization (JSON/binary) | `serde` (+ `serde_json`, or `bincode` for compact binary) |
| Error handling | `anyhow` (apps), `thiserror` (libs) |
| Logging | `tracing` (+ `tracing-subscriber`) |
| Random (loot, spawns) | `rand` |
| Full game engine (optional) | `bevy` (ECS + rendering, big) |
| Lightweight ECS only | `hecs` |
| Fast hash maps | `dashmap` (concurrent), `ahash` |

## Tests live alongside code

```rust
pub fn attack(power: u32, defense: u32) -> u32 { power.saturating_sub(defense) }

#[cfg(test)]                  // only compiled when testing
mod tests {
    use super::*;
    #[test]
    fn attack_subtracts_defense() {
        assert_eq!(attack(10, 3), 7);
    }
    #[test]
    fn attack_floors_at_zero() {
        assert_eq!(attack(2, 10), 0);
    }
}
```

Run with `cargo test`. Integration tests go in a top-level `tests/` directory. Build the habit
early — tests are how you keep a growing game from regressing.

## Useful cargo workflow recap

```bash
cargo check          # fast type-check loop
cargo run --bin server
cargo test
cargo clippy         # lint
cargo fmt            # format
cargo doc --open     # build & view your docs
cargo build --release   # optimized build for actually running the game
```

---

🏋️ Do `Exercises/08_modules_project.md`.

➡️ Next: `09_smart_pointers.md`
