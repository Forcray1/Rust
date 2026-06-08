# Exercises 14 — ECS & Game Loop

Organize a game the way real engines do. You'll hand-roll a mini-ECS so the concept is concrete.

## Mini-project: a hand-rolled mini-ECS

Build a tiny ECS with three component types and three systems, then run a tick loop.

```rust
use std::collections::HashMap;

type Entity = u32;

#[derive(Clone, Copy, Debug)] struct Position { x: f32, y: f32 }
#[derive(Clone, Copy, Debug)] struct Velocity { dx: f32, dy: f32 }
#[derive(Clone, Copy, Debug)] struct Health   { hp: i32 }

#[derive(Default)]
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

// SYSTEM 1: integrate motion for entities that have BOTH position and velocity.
fn movement_system(w: &mut World, dt: f32) {
    // TODO
}

// SYSTEM 2: everything with a position takes 1 damage per tick if it has health (an "environment").
fn hazard_system(w: &mut World) {
    // TODO
}

// SYSTEM 3: despawn entities whose hp <= 0 (remove from ALL component maps).
fn death_system(w: &mut World) {
    // TODO  (collect dead ids first, then remove)
}

fn main() {
    let mut w = World::default();

    // a moving, living entity
    let player = w.spawn();
    w.positions.insert(player, Position { x: 0.0, y: 0.0 });
    w.velocities.insert(player, Velocity { dx: 1.0, dy: 0.5 });
    w.healths.insert(player, Health { hp: 3 });

    // a stationary rock (position only — no velocity, no health)
    let rock = w.spawn();
    w.positions.insert(rock, Position { x: 5.0, y: 5.0 });

    // run 5 ticks
    for tick in 1..=5 {
        movement_system(&mut w, 1.0);
        hazard_system(&mut w);
        death_system(&mut w);
        println!("-- after tick {tick} --");
        for (id, pos) in &w.positions {
            println!("  entity {id} at ({:.1}, {:.1})", pos.x, pos.y);
        }
    }
}
```

**Expected behavior:** the player moves each tick and loses 1 hp/tick (despawned after tick 3 when
hp hits 0); the rock stays put forever (no health → never dies, no velocity → never moves). Notice
how systems only touch entities that have the *right combination* of components — that's ECS.

**Done when:** the three systems work, the player despawns at the right time, and the rock persists.

## Stretch goals (toward the real game)

1. Add a `Name(String)` component and a system that prints `"<name> died"` when an entity is about
   to be removed.
2. Add an `combat_system`: entities with a `Target(Entity)` component deal damage to their target's
   `Health`. Watch how "add a component + a system" extends the game without touching existing
   systems.
3. Swap your hand-rolled storage for the **`hecs`** crate (`cargo add hecs`) and reimplement
   `movement_system` as a `world.query_mut::<(&mut Position, &Velocity)>()` loop. Compare ergonomics.

---
<details><summary>Solution sketch</summary>

```rust
fn movement_system(w: &mut World, dt: f32) {
    for (id, vel) in &w.velocities {
        if let Some(pos) = w.positions.get_mut(id) {
            pos.x += vel.dx * dt;
            pos.y += vel.dy * dt;
        }
    }
}
fn hazard_system(w: &mut World) {
    for (_id, h) in w.healths.iter_mut() {
        h.hp -= 1;
    }
}
fn death_system(w: &mut World) {
    let dead: Vec<Entity> = w.healths.iter()
        .filter(|(_, h)| h.hp <= 0).map(|(id, _)| *id).collect();
    for id in dead {
        w.positions.remove(&id);
        w.velocities.remove(&id);
        w.healths.remove(&id);
    }
}
```
(Borrow note: in `movement_system` we iterate `&w.velocities` while mutating `w.positions` — two
*different* maps, so the borrow checker is happy. Iterating and mutating the *same* map would not
compile — collect-then-mutate, like `death_system` does.)
</details>
