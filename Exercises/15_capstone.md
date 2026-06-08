# Exercises 15 — Capstone: Multiplayer Movement + Chat

This builds Stages 0→4 from `Courses/15_capstone_toward_mmorpg.md` into a real, working multiplayer
demo: multiple clients connect, move around a 2D world, see each other's positions update in
real time, and chat. This is the genuine foundation of an MMORPG. Everything you learned shows up
here.

> Take this in stages. Each stage runs on its own. Don't try to write it all at once.

## Project setup

```bash
cargo new mygame
cd mygame
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add tokio-util --features codec
cargo add futures
cargo add anyhow
```

Layout (module 08):
```
src/
├── lib.rs          # protocol types shared by server + client
├── bin/server.rs
└── bin/client.rs
```

## The shared protocol — `src/lib.rs`

```rust
use serde::{Serialize, Deserialize};

pub type EntityId = u32;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Pos { pub x: f32, pub y: f32 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    Login { name: String },
    Move { dx: f32, dy: f32 },
    Chat(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerMessage {
    Welcome { id: EntityId },
    State { players: Vec<(EntityId, String, Pos)> },
    Chat { from: String, text: String },
}
```

We'll send these as **newline-delimited JSON** (`LinesCodec`) so you can read the wire by eye while
learning. (Swap to `LengthDelimitedCodec` + `bincode` later for a compact binary protocol — only
the encode/decode lines change.)

## Server architecture (the shape to build)

```
accept loop ──▶ per-connection task ──(mpsc: ServerEvent)──▶ game task (owns World)
                       ▲                                            │
                       └────────(broadcast: ServerMessage)──────────┘
                                                          tick @ 20Hz: apply moves, broadcast State
```

- **Per-connection task**: reads JSON lines → `ClientMessage`, forwards them to the game task as
  `ServerEvent`s (tagged with the player's id). Also `subscribe`s to the broadcast channel and
  writes any `ServerMessage` out to its client. Use `select!` to do both.
- **Game task**: the single owner of the `World` (`HashMap<EntityId, Player>`). Processes events,
  runs a 20Hz tick, broadcasts `State`.

`ServerEvent` (server-internal, not on the wire):
```rust
enum ServerEvent {
    Join { id: EntityId, name: String },
    Input { id: EntityId, msg: ClientMessage },
    Leave { id: EntityId },
}
```

## Build order

### Stage 0–1: framed JSON echo
Get a server that accepts a connection, reads a JSON `ClientMessage` line, prints it, and replies
with `ServerMessage::Welcome { id }`. Write a client that connects, sends `Login`, and prints the
`Welcome`. ✅ when the typed round-trip works.

### Stage 2: track players
On `Login`, the game task assigns an `EntityId`, inserts a `Player { name, pos: (0,0) }` into the
world, and sends `Welcome`. On disconnect, remove the player. Log the player count. ✅ when two
clients connect and the server logs both, and removes them on quit.

### Stage 3: broadcast chat
Add a `broadcast::channel::<ServerMessage>`. On `ClientMessage::Chat`, the game task broadcasts
`ServerMessage::Chat { from, text }`. Each connection task forwards broadcasts to its client. ✅
when a chat typed in client A prints in client B.

### Stage 4: movement + tick loop
On `ClientMessage::Move { dx, dy }`, update that player's `Pos`. A 20Hz `interval` in the game task
broadcasts `ServerMessage::State { players }` every tick. The client prints positions when it
receives `State`. ✅ when moving in one client changes the position other clients see.

## Reference: game task core (Stage 4)

```rust
// inside the game task — owns `world`, holds the broadcast sender `bcast`, reads events from `rx`
use tokio::time::{interval, Duration};

let mut ticker = interval(Duration::from_millis(50));   // 20 Hz
loop {
    tokio::select! {
        _ = ticker.tick() => {
            let players: Vec<_> = world.iter()
                .map(|(id, p)| (*id, p.name.clone(), p.pos))
                .collect();
            let _ = bcast.send(ServerMessage::State { players });
        }
        Some(event) = rx.recv() => {
            match event {
                ServerEvent::Join { id, name } => { world.insert(id, Player { name, pos: Pos { x: 0.0, y: 0.0 } }); }
                ServerEvent::Leave { id } => { world.remove(&id); }
                ServerEvent::Input { id, msg } => match msg {
                    ClientMessage::Move { dx, dy } => {
                        if let Some(p) = world.get_mut(&id) { p.pos.x += dx; p.pos.y += dy; }
                    }
                    ClientMessage::Chat(text) => {
                        let from = world.get(&id).map(|p| p.name.clone()).unwrap_or_default();
                        let _ = bcast.send(ServerMessage::Chat { from, text });
                    }
                    ClientMessage::Login { .. } => {} // already handled at connect
                },
            }
        }
    }
}
```

## Reference: per-connection task core

```rust
// `framed` = Framed<TcpStream, LinesCodec>; `to_game` = mpsc::Sender<ServerEvent>;
// `mut updates` = broadcast::Receiver<ServerMessage>; `id` assigned at login.
loop {
    tokio::select! {
        line = framed.next() => {
            match line {
                Some(Ok(json)) => {
                    if let Ok(msg) = serde_json::from_str::<ClientMessage>(&json) {
                        let _ = to_game.send(ServerEvent::Input { id, msg }).await;
                    } // malformed line -> just ignore it; never crash
                }
                _ => break,   // disconnect
            }
        }
        Ok(update) = updates.recv() => {
            let json = serde_json::to_string(&update).unwrap();
            if framed.send(json).await.is_err() { break; }
        }
    }
}
let _ = to_game.send(ServerEvent::Leave { id }).await;
```

## The client (simple version)

Connect, spawn a task that reads stdin lines and turns them into `ClientMessage`s
(`"say hello"` → Chat, `"move 1 0"` → Move), and a task that reads `ServerMessage`s and prints
them. Two clients + your keyboard = a tiny MMO.

(Stdin in async: `tokio::io::BufReader::new(tokio::io::stdin()).lines()`.)

## Done when (the finish line for this whole course)

- [ ] Two or more clients connect simultaneously.
- [ ] Each client can `move` and sees its own and others' positions update ~20×/sec.
- [ ] Chat from one client appears on all clients.
- [ ] A client disconnecting is removed from the world and doesn't crash the server.
- [ ] Malformed input is ignored, not fatal.

If you've got this running, **you have built the core of an MMORPG server** — authoritative state,
many concurrent connections, a tick loop, a real protocol, robust to bad input. From here it's
*game design*: combat, items, a map, persistence, and eventually a graphical client (bevy). All of
it is "add an enum variant + a component + a system," using exactly the skills you now have.

## Where to extend next

- **Combat**: `Health` per player, `ClientMessage::Attack { target }`, a `combat_system` in the
  tick, broadcast deaths/respawns.
- **A real map**: grid + collision; reject moves into walls server-side (authoritative).
- **Interest management**: only send each client the players *near* them (scales to many players).
- **Binary protocol**: switch `LinesCodec`+JSON → `LengthDelimitedCodec`+`bincode`.
- **Persistence**: save/load the world and accounts with serde.
- **Graphical client**: rebuild the client in **bevy**, reusing `lib.rs` as the shared protocol.

🎉 That's the path from "what do I need to start" to "I built a multiplayer game server in Rust."
