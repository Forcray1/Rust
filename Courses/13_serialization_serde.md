# 13 — Serialization with serde

Networking moves *bytes*; your game logic works with *structs and enums*. Serialization is the
bridge: turn a `ClientMessage` into bytes to send, and bytes back into a `ServerMessage` on
receipt. In Rust this is delightfully easy thanks to **serde**.

## serde in one picture

```
   your types  ──Serialize──▶   bytes (JSON / binary)   ──network──▶  bytes  ──Deserialize──▶  your types
```

serde is *format-agnostic*: the same `#[derive]` works for JSON, binary (bincode), MessagePack,
TOML, etc. You pick the format crate.

## Setup

```bash
cargo add serde --features derive
cargo add serde_json        # human-readable JSON (great while learning/debugging)
cargo add bincode           # compact binary (great for the real wire protocol)
```

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "1"
```

## Make your types serializable

Just derive `Serialize` and `Deserialize`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Position { x: f32, y: f32 }

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ClientMessage {
    Login { username: String },
    Move { dx: f32, dy: f32 },
    Chat(String),
    Attack { target_id: u32 },
    Logout,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ServerMessage {
    Welcome { your_id: u32 },
    PlayerMoved { id: u32, pos: Position },
    ChatBroadcast { from: String, text: String },
    PlayerLeft { id: u32 },
    Error(String),
}
```

That `#[derive]` generates all the (de)serialization code at compile time — no reflection, no
runtime cost like Python's `pickle`/`json` introspection.

## JSON (readable — use while developing)

```rust
let msg = ClientMessage::Chat("hello world".into());

let json: String = serde_json::to_string(&msg)?;
// -> {"Chat":"hello world"}
println!("{json}");

let back: ClientMessage = serde_json::from_str(&json)?;
```

JSON is perfect early on: you can read packets in your terminal/logs and debug by eye. Pair with
`LinesCodec` (newline-delimited JSON) from module 12.

## Binary (compact — use for the real protocol)

```rust
let bytes: Vec<u8> = bincode::serialize(&msg)?;     // small, fast
let back: ClientMessage = bincode::deserialize(&bytes)?;
```

bincode produces a tight binary blob — far smaller than JSON, ideal for high-frequency packets like
movement. Pair with `LengthDelimitedCodec` from module 12. A clean approach: develop with JSON,
switch to bincode for performance once the protocol is stable (the code barely changes — just the
format crate).

## Tying serde to the network (the full round trip)

```rust
// SENDING
fn encode(msg: &ClientMessage) -> Vec<u8> {
    bincode::serialize(msg).expect("serialization should not fail")
}
// ... then write length-prefixed `encode(&msg)` to the socket (module 12)

// RECEIVING (after the codec hands you one complete frame of bytes):
fn decode(bytes: &[u8]) -> Result<ClientMessage, bincode::Error> {
    bincode::deserialize(bytes)
}
```

Because deserialization **returns a `Result`** (module 04), malformed/hostile input becomes a
clean `Err` you handle — not a crash. This is how a server stays up under garbage input: every
incoming packet is `decode(...)?` and a bad one just drops that message (or the connection),
never the server.

## Handy serde attributes

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default)]                  // missing field -> Default value
    max_players: u32,

    #[serde(rename = "playerName")]    // different name on the wire
    player_name: String,

    #[serde(skip)]                     // never (de)serialize this field
    runtime_only: Cache,
}
```

These let your wire format and your internal types diverge cleanly (versioning, optional fields,
backward compatibility as your protocol evolves).

## Saving game state to disk (same tool)

serde isn't only for networking — persist the world the same way:

```rust
let json = serde_json::to_string_pretty(&world)?;
std::fs::write("save.json", json)?;

let world: World = serde_json::from_str(&std::fs::read_to_string("save.json")?)?;
```

So player accounts, world snapshots, and config files all use the one mechanism you already know.

## Why this matters for the MMORPG

- **One enum per direction** (`ClientMessage`, `ServerMessage`) = your entire protocol, type-checked
  end to end. Add a message type → the compiler shows every `match` that must handle it (module 03).
- **serde + the codec from module 12** = a complete, safe message pipeline in ~20 lines.
- **Bad input is a `Result`, not a crash** — exactly the robustness a server needs.

---

🏋️ Do `Exercises/13_serialization.md`.

➡️ Next: `14_game_architecture_ecs.md`
