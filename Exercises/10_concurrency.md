# Exercises 10 — Concurrency: Threads & Channels

Fearless concurrency in practice. Two architectures, both of which you'll recognize from the server
design.

## Drills

**D1.** Spawn 5 threads, each printing its index (0–4). `join` them all. Note that output order is
nondeterministic — that's concurrency.

**D2.** The classic shared counter: spawn 10 threads, each incrementing an `Arc<Mutex<i32>>` 1000
times. After joining, assert the total is 10_000. Remove the Mutex (try a plain `Arc<i32>`) and
observe the compiler refuse — it won't let you create the data race.

**D3.** Move semantics: try to use a `Vec` in a spawned thread *without* `move`, read the error,
then fix it with `move`. Explain why the thread needs ownership.

## Mini-project A: shared-world tick (Arc<Mutex>)

Simulate several "connection" threads mutating a shared world while a "tick" loop reads it.

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

struct World { players: HashMap<u32, u32> }   // id -> hp

fn main() {
    let world = Arc::new(Mutex::new(World { players: HashMap::new() }));
    let mut handles = vec![];

    // 4 "players" join and take some damage over time.
    for id in 0..4 {
        let w = Arc::clone(&world);
        handles.push(thread::spawn(move || {
            { w.lock().unwrap().players.insert(id, 100); }
            for _ in 0..10 {
                let mut g = w.lock().unwrap();
                if let Some(hp) = g.players.get_mut(&id) {
                    *hp = hp.saturating_sub(5);
                }
            }
        }));
    }

    for h in handles { h.join().unwrap(); }

    let g = world.lock().unwrap();
    for (id, hp) in &g.players {
        println!("player {id}: {hp} hp");   // each should be 50
    }
}
```

Run it a few times — results are stable (50 each) because the Mutex serializes access. *That's the
point.*

## Mini-project B: message-passing game loop (channels) — the recommended architecture

Reframe the same idea without locks: connection threads *send events*; one game thread *owns* the
world and processes them.

```rust
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

enum Event { Join(u32), Damage(u32, u32), Quit }

fn main() {
    let (tx, rx) = mpsc::channel::<Event>();

    // "connection" threads produce events:
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(Event::Join(id)).unwrap();
            tx.send(Event::Damage(id, 30)).unwrap();
        });
    }
    drop(tx);   // close channel once all senders are gone

    // the GAME THREAD owns the world — no Mutex needed:
    let mut world: HashMap<u32, u32> = HashMap::new();
    for event in rx {
        match event {
            Event::Join(id) => { world.insert(id, 100); }
            Event::Damage(id, dmg) => {
                if let Some(hp) = world.get_mut(&id) { *hp = hp.saturating_sub(dmg); }
            }
            Event::Quit => break,
        }
    }

    let mut ids: Vec<_> = world.keys().collect();
    ids.sort();
    for id in ids { println!("player {id}: {} hp", world[id]); }   // each 70
}
```

**Done when:** both run correctly. Then answer for yourself: *why is version B easier to reason
about than version A?* (No locks; the world has exactly one owner; all changes flow through one
serialized event stream — exactly architecture B from the course, and what the capstone uses.)

---
<details><summary>Hints</summary>

- `Arc::clone(&x)` before `move` into each thread — each thread needs its own handle.
- A scope `{ ... }` around a `lock()` releases it promptly (the guard drops at `}`). Holding locks
  too long is the #1 source of contention.
- In version B, you must `drop(tx)` (and ensure clones are dropped when threads end) or the `for
  event in rx` loop never terminates — it waits for more senders.
</details>
