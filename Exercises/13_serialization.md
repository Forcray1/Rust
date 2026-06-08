# Exercises 13 — Serialization (serde)

Turn your protocol types into bytes and back.

Setup: `cargo add serde --features derive` and `cargo add serde_json bincode`.

## Drills

**D1.** Add `#[derive(Serialize, Deserialize, Debug, PartialEq)]` to a `Position { x: f32, y: f32 }`
struct. Round-trip it through JSON:
```rust
let p = Position { x: 1.5, y: -2.0 };
let json = serde_json::to_string(&p)?;
println!("{json}");
let back: Position = serde_json::from_str(&json)?;
assert_eq!(p, back);
```

**D2.** Do the same with **bincode** (`bincode::serialize` / `deserialize`). Print the byte length
of the bincode output vs the JSON string length — note how much smaller binary is.

**D3.** Serialize an *enum* (`ClientMessage` from module 03/13) to JSON. Look at the JSON for each
variant — see how serde encodes struct-variants vs tuple-variants vs unit-variants.

**D4.** Deserialize deliberately broken JSON (e.g. `{"x": "not a number"}`) into `Position` and
confirm you get an `Err`, not a panic. Print the error. *This is why servers don't crash on bad
packets.*

## Mini-project: the full protocol round-trip

Define the real two-way protocol and a `encode`/`decode` pair you can drop into the networking
code.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
struct Pos { x: f32, y: f32 }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ClientMessage {
    Login { name: String },
    Move { dx: f32, dy: f32 },
    Chat(String),
    Logout,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ServerMessage {
    Welcome { id: u32 },
    State { players: Vec<(u32, String, Pos)> },
    Chat { from: String, text: String },
    PlayerLeft { id: u32 },
}

fn encode<T: Serialize>(msg: &T) -> Vec<u8> {
    bincode::serialize(msg).expect("serialize")
}
fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, bincode::Error> {
    bincode::deserialize(bytes)
}

fn main() {
    // 1. Round-trip a ClientMessage::Move through encode/decode, assert equality.
    // 2. Round-trip a ServerMessage::State carrying 2 players.
    // 3. Feed decode::<ClientMessage> some random bytes (e.g. &[9, 9, 9, 255]) and confirm Err.
    // 4. Print JSON of a State message (serde_json::to_string_pretty) to read it by eye.
}
```

**Done when:** all four steps work, the corrupt-bytes case returns `Err` (no panic), and you
understand that `encode`/`decode` are generic over *any* serde type — so the same two functions
serialize your entire protocol. These plug directly into the `LengthDelimitedCodec` from module 12.

---
<details><summary>Solution sketch</summary>

```rust
// 1.
let m = ClientMessage::Move { dx: 1.0, dy: -2.0 };
assert_eq!(decode::<ClientMessage>(&encode(&m)).unwrap(), m);
// 2.
let s = ServerMessage::State { players: vec![
    (1, "Aria".into(), Pos { x: 0.0, y: 0.0 }),
    (2, "Bob".into(),  Pos { x: 3.0, y: 4.0 }),
]};
assert_eq!(decode::<ServerMessage>(&encode(&s)).unwrap(), s);
// 3.
assert!(decode::<ClientMessage>(&[9, 9, 9, 255]).is_err());
// 4.
println!("{}", serde_json::to_string_pretty(&s).unwrap());
```
The `for<'de>` in `decode`'s signature is a "higher-ranked trait bound" — read it as "T can be
deserialized from borrowed data of any lifetime." You rarely write it; serde's docs have the
pattern.
</details>
