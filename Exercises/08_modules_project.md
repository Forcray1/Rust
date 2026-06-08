# Exercises 08 — Modules, Crates & Cargo

This one is hands-on with real files, not a single `main.rs`.

## Drill / Mini-project: split a project into a lib + two bins

Create a fresh project and structure it the way the MMORPG will be structured.

```bash
cargo new mygame
cd mygame
```

Build this layout:

```
mygame/
├── Cargo.toml
└── src/
    ├── lib.rs              # the shared library
    ├── protocol.rs        # mod protocol — ClientMessage / ServerMessage enums
    ├── entity.rs          # mod entity — Player struct + impl
    └── bin/
        ├── server.rs      # uses mygame::{protocol, entity}
        └── client.rs      # uses mygame::protocol
```

**Tasks:**

1. In `src/lib.rs`, declare the modules and re-export their key types:
   ```rust
   pub mod protocol;
   pub mod entity;
   ```
2. In `src/protocol.rs`, define `pub enum ClientMessage { ... }` and `pub enum ServerMessage { ... }`
   (reuse module 03's). Mark them and their fields `pub`.
3. In `src/entity.rs`, define `pub struct Player` with a `pub fn new(...)` and a private helper.
   Confirm the private helper is **not** accessible from `server.rs`.
4. In `src/bin/server.rs`, write a `main` that creates a `Player`, constructs a couple of
   `ClientMessage`s, and prints them. Import with `use mygame::protocol::ClientMessage;` etc.
5. In `src/bin/client.rs`, write a `main` that builds a `ClientMessage::Login` and prints it.

**Run:**
```bash
cargo run --bin server
cargo run --bin client
cargo check          # should type-check everything
```

**Stretch:**
- Add `cargo add anyhow` and make both `main`s return `anyhow::Result<()>`.
- Add a `#[cfg(test)] mod tests` in `entity.rs` testing `Player::new`, and run `cargo test`.
- Try making a field private and observe the compile error when `server.rs` touches it directly —
  this is module boundaries enforced by the compiler.

**Done when:** both binaries build and run, the lib is shared between them, and you've seen the
visibility rules reject access to a private item. You now have the exact skeleton the capstone uses.

---
<details><summary>Solution sketch</summary>

`src/lib.rs`:
```rust
pub mod protocol;
pub mod entity;
```
`src/bin/server.rs`:
```rust
use mygame::protocol::ClientMessage;
use mygame::entity::Player;

fn main() {
    let p = Player::new("Aria".to_string());
    let msgs = vec![
        ClientMessage::Login { username: "Aria".into() },
        ClientMessage::Chat("hi".into()),
    ];
    println!("{p:?}");
    for m in &msgs { println!("{m:?}"); }
}
```
The crate name (`mygame`) comes from `Cargo.toml`'s `[package] name`. Binaries in `src/bin/`
reference the library by that name.
</details>
