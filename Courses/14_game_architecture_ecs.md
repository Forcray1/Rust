# 14 — Game Architecture & ECS

You now know the language. This module is about *organizing* a game so it stays maintainable and
fast. The dominant pattern in modern game dev — and a natural fit for Rust's "composition over
inheritance" philosophy (module 05) — is **ECS: Entity-Component-System**.

## Why not the obvious OOP design?

The instinct from other languages: a class hierarchy.

```
Entity → Creature → Humanoid → Player
                 → Monster
       → Item
```

This breaks down fast: where does a "talking sword" go? What about a monster that can be looted
like an item? Inheritance forces a single rigid tree. Rust doesn't even have inheritance — and
that's a feature here.

## ECS: the core idea

Split the game into three things:

- **Entity** — just an id. A `u32`. It owns no data itself; it's a label.
- **Component** — a plain data struct attached to an entity: `Position`, `Health`, `Velocity`,
  `Inventory`, `PlayerControlled`, `Sprite`. No behavior, just data.
- **System** — a function that runs over all entities having a given set of components, each tick.
  `movement_system` runs over everything with `Position + Velocity`. `combat_system` over things
  with `Health`. Etc.

An entity *is* whatever components it has. A "talking sword" = an entity with `Position`,
`Item`, and `Dialogue` components. No hierarchy needed — pure composition.

```
Entity 1: [Position, Velocity, Health, PlayerControlled]   ← a player
Entity 2: [Position, Velocity, Health, AI]                 ← a monster
Entity 3: [Position, Item]                                 ← dropped loot
Entity 4: [Position, Health, AI, Vendor, Dialogue]         ← an NPC shopkeeper
```

## A hand-rolled mini-ECS (to understand it)

You can grasp ECS with plain `HashMap`s before reaching for a library:

```rust
use std::collections::HashMap;

type Entity = u32;

#[derive(Clone, Copy)] struct Position { x: f32, y: f32 }
#[derive(Clone, Copy)] struct Velocity { dx: f32, dy: f32 }
#[derive(Clone, Copy)] struct Health  { hp: i32 }

struct World {
    next_id: Entity,
    positions:  HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    healths:    HashMap<Entity, Health>,
}

impl World {
    fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// A SYSTEM: move every entity that has both a position and a velocity.
fn movement_system(world: &mut World, dt: f32) {
    for (id, vel) in &world.velocities {
        if let Some(pos) = world.positions.get_mut(id) {
            pos.x += vel.dx * dt;
            pos.y += vel.dy * dt;
        }
    }
}

// Another SYSTEM: remove the dead.
fn death_system(world: &mut World) {
    let dead: Vec<Entity> = world.healths.iter()
        .filter(|(_, h)| h.hp <= 0)
        .map(|(id, _)| *id)
        .collect();
    for id in dead {
        world.positions.remove(&id);
        world.velocities.remove(&id);
        world.healths.remove(&id);
    }
}
```

Notice this is just modules 03, 06, and 02 applied: structs as components, iterators to query,
`get_mut` + borrow rules to mutate safely. **ECS is not new magic — it's the Rust you already know,
organized well.**

## The game loop (where systems run)

```rust
fn run(world: &mut World) {
    let dt = 0.05;                 // 50ms per tick = 20 ticks/sec
    loop {
        // 1. ingest player input (from the network channel — modules 11/12)
        // 2. run systems in order:
        ai_system(world);
        input_system(world);
        movement_system(world, dt);
        combat_system(world);
        death_system(world);
        // 3. broadcast new state to clients (serde + network — modules 12/13)
        // 4. sleep until next tick (tokio interval — module 11)
    }
}
```

A game is fundamentally this: **a loop that, each tick, runs a pipeline of systems over the world,
then syncs state to clients.** Everything else is detail.

## Use a real ECS library for the actual game

Hand-rolling teaches you the model; for the real thing, use a library that stores components
efficiently (contiguous arrays → cache-friendly → fast) and handles queries for you:

| Crate | What it is |
|-------|-----------|
| **`hecs`** | a lean, standalone ECS — great for a server with no rendering |
| **`bevy`** | a full game engine (ECS + rendering + input + audio) — great if you also want a graphical client |
| `legion`, `specs` | other mature ECS options |

`hecs` query example (conceptually the same as your hand-rolled systems, but fast and ergonomic):

```rust
// for (id, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>() {
//     pos.x += vel.dx * dt;
//     pos.y += vel.dy * dt;
// }
```

## How the MMORPG pieces fit

```
            ┌──────────────────────── server ────────────────────────┐
 clients ──▶│ net tasks (mod 11/12)  ──actions──▶  game loop (this)   │
   ◀────────│         ▲                            owns ECS World     │
            │         └──── state broadcasts ◀──── systems run/tick   │
            └─────────────────────────────────────────────────────────┘
            packets (de)serialized with serde (mod 13)
            shared/owned state per mod 02/09/10
```

- **Networking layer** (Tokio): accept connections, decode `ClientMessage`s, push them into the
  game loop via a channel.
- **Game loop / ECS** (this module): owns the `World`, runs systems each tick, produces
  `ServerMessage`s.
- **Serialization** (serde): converts between messages and bytes at the boundary.

Each is a module/crate boundary (module 08). Clean separation = a codebase you can actually grow.

## Design advice for your first version

1. **Single-threaded game loop, owned World** — don't add locks until you must. One game task owns
   the world; networking talks to it via channels (architecture B from module 10).
2. **Fixed tick rate** (e.g. 20/sec). Simpler than variable timestep; fine to start.
3. **Authoritative server**: the server owns truth; clients send *intent* ("I want to move left"),
   server decides and broadcasts results. Never trust the client. This prevents cheating and
   simplifies sync.
4. **Start tiny**: a world with positions and chat, players that move on a grid, see each other,
   and talk. Add combat, items, persistence incrementally.

---

🏋️ Do `Exercises/14_ecs_gameloop.md`.

➡️ Next: `15_capstone_toward_mmorpg.md` — put it all together.
