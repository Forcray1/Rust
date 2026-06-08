# Exercises 03 — Structs, Enums & Pattern Matching

You'll build the seed of your network protocol here.

## Drills

**D1.** Define a `Player` struct (`name: String`, `hp: u32`, `level: u8`, `pos: (f32, f32)`).
Add an `impl` with `new(name)`, `is_alive(&self) -> bool`, and `take_damage(&mut self, u32)` that
floors at 0 (use `saturating_sub`). Derive `Debug` and print a player with `{:?}`.

**D2.** Define an enum `Tile { Empty, Wall, Water, Door { locked: bool } }`. Write
`fn walkable(t: &Tile) -> bool` using `match` (water and walls block; an unlocked door is walkable).

**D3.** Rewrite this `match` as an `if let`:
```rust
match maybe_id { Some(id) => println!("id {id}"), None => {} }
```

**D4.** Use a match guard: `fn rank(score: u32) -> &'static str` returning `"S"` for ≥90, `"A"` for
≥75, `"B"` for ≥50, else `"C"`.

## Mini-project: a packet enum + dispatcher

Model a tiny client→server protocol and a handler that pattern-matches on it.

```rust
#[derive(Debug)]
enum ClientMessage {
    Login { username: String },
    Move { dx: f32, dy: f32 },
    Chat(String),
    Attack { target_id: u32 },
    Logout,
}

// TODO: implement this. Return a human-readable description of the action.
fn handle(msg: &ClientMessage) -> String {
    todo!()
}

fn main() {
    let messages = vec![
        ClientMessage::Login { username: "Aria".into() },
        ClientMessage::Move { dx: 1.0, dy: -0.5 },
        ClientMessage::Chat("hello!".into()),
        ClientMessage::Attack { target_id: 7 },
        ClientMessage::Logout,
    ];
    for m in &messages {
        println!("{}", handle(m));
    }
}
```

**Stretch:** add a new variant `Emote(String)`. Notice the compiler now *forces* you to update
`handle` (exhaustiveness). This is the property that keeps a big protocol correct.

**Done when:** every message prints a sensible description, and you've experienced the compiler
flagging the missing `Emote` case.

---
<details><summary>Solution sketch</summary>

```rust
fn handle(msg: &ClientMessage) -> String {
    match msg {
        ClientMessage::Login { username } => format!("{username} logged in"),
        ClientMessage::Move { dx, dy } => format!("moved by ({dx}, {dy})"),
        ClientMessage::Chat(text) => format!("says: {text}"),
        ClientMessage::Attack { target_id } => format!("attacks entity {target_id}"),
        ClientMessage::Logout => "logged out".to_string(),
    }
}
```
D2: `matches!(t, Tile::Empty) || matches!(t, Tile::Door { locked: false })` — or a full match.
</details>
