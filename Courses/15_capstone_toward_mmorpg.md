# 15 — Capstone: Toward the MMORPG

By now you've learned every ingredient. This module doesn't teach new syntax — it shows how the
pieces snap together and gives you a **staged build plan** from "hello socket" to a real little
online game. Each stage is a working program; you never build the whole thing at once.

> Reminder of the goal: you don't have to finish an MMORPG here. By the end of the exercises you'll
> have the knowledge and a working multiplayer foundation you can keep extending.

## The full ingredient → module map

| Ingredient | Modules |
|-----------|---------|
| Model players, items, packets as data | 03 (structs/enums), 13 (serde) |
| Never crash on bad input | 04 (Result/Option) |
| Shared/owned world state | 02 (ownership), 09 (Arc/Mutex) |
| Many connections at once | 10 (threads/channels), 11 (async/Tokio) |
| The wire | 12 (TCP + framing), 13 (serde) |
| Game loop over entities | 06 (iterators), 14 (ECS) |
| Reusable, organized code | 05 (traits), 08 (modules/crates) |

## The protocol (shared library)

Put this in a `lib.rs` shared by both server and client binaries (module 08):

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
    Logout,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerMessage {
    Welcome { id: EntityId },
    State { players: Vec<(EntityId, String, Pos)> },
    Chat { from: String, text: String },
    PlayerLeft { id: EntityId },
}
```

That's your entire network contract — type-checked on both ends.

## The staged build plan

### Stage 0 — Echo (module 12)
TCP server that echoes bytes; a client that connects and sends a line. Goal: prove your toolchain
and the accept→spawn→read/write loop work. ✅ when `nc` echoes.

### Stage 1 — Framed JSON messages (modules 12, 13)
Swap raw bytes for `ClientMessage`/`ServerMessage` over a `LinesCodec` (newline-delimited JSON).
Server logs every decoded message; replies with `Welcome`. ✅ when client sends `Login` and sees
`Welcome` come back as a typed value.

### Stage 2 — Many clients + a shared player list (modules 09, 10/11)
Spawn a task per connection. Maintain a shared `HashMap<EntityId, Player>` (start with
`Arc<Mutex<...>>`, or better, a game task + channels — architecture B, module 10). On login, assign
an id and add the player. On disconnect, remove them. ✅ when two clients connect and the server
knows about both.

### Stage 3 — Broadcast chat (modules 11, 13)
Use a `tokio::sync::broadcast` channel. When any client sends `Chat`, broadcast `ServerMessage::
Chat` to all. Each connection task `select!`s between "read from my socket" and "receive a
broadcast to forward to my client" (module 11). ✅ when a message typed in client A appears in
client B.

### Stage 4 — Movement + tick loop (modules 11, 14)
Add `Pos` to each player. Client sends `Move { dx, dy }`. A game task runs a 20Hz tick: apply
queued moves, then broadcast `State { players }` to everyone. ✅ when moving in one client updates
the position seen by others.

### Stage 5 — ECS refactor (module 14)
Replace ad-hoc `HashMap`s with a small ECS (hand-rolled or `hecs`). Express movement, and later
combat, as systems. ✅ when adding a new mechanic = adding a component + a system, touching nothing
else.

### Stage 6+ — Make it a game
Now iterate on *game design* with the same tools:
- Combat: `Health` component, `Attack` message, `combat_system`, death/respawn.
- World: a grid/map, collision, zones.
- Items & inventory: `Item` entities, pickup, `Inventory` component.
- Persistence: save/load world and accounts with serde (module 13).
- A graphical client: adopt **bevy** (module 14) for rendering while reusing your protocol lib.

Each addition is "a new enum variant + a component + a system" — and the compiler guides every
change.

## Architecture recap (the shape to aim for)

```
mygame/
├── src/lib.rs            # protocol (ClientMessage/ServerMessage), World, components, systems
├── src/bin/server.rs     # Tokio: accept loop, per-connection tasks, game-loop task
└── src/bin/client.rs     # Tokio: connect, send input, render/print state
```

```
server.rs
  ├─ accept loop ──▶ per-connection task ──(mpsc)──▶ game task (owns World/ECS)
  │                          ▲                              │
  │                          └──────(broadcast)─────────────┘  pushes State each tick
  └─ game task: loop { tick(); apply actions; broadcast state }
```

- **Authoritative server**: clients send intent, server decides truth, broadcasts results.
- **Owned world in one task**: no locks needed for the world itself; channels carry everything in
  and out.
- **One protocol lib**: client and server can't disagree about message formats — it won't compile.

## Where to go after this

- Read **The Rust Book** end to end to fill any gaps: https://doc.rust-lang.org/book/
- Do **Rustlings** for extra reps: https://github.com/rust-lang/rustlings
- Explore **bevy** if you want graphics: https://bevyengine.org/
- Study an open-source Rust game server (e.g. **Veloren** is a real MMORPG-ish game in Rust) to see
  these patterns at scale: https://gitlab.com/veloren/veloren

## You're ready

You now know: ownership/borrowing, the type system (structs/enums/traits/generics/lifetimes),
error handling, collections/iterators, concurrency, async networking, serialization, and game
architecture. That is the complete toolkit to build the MMORPG. Build it stage by stage — the
exercises below take you most of the way.

---

🏋️ Final exercises: `Exercises/15_capstone.md` — builds Stages 0→4 into a working multiplayer demo.
