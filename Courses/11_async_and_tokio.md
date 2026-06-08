# 11 — Async & Tokio ⭐

This is how real network servers handle thousands of simultaneous connections. It's a genuinely
new mental model (Python has `asyncio`, which is the closest analogy; C has nothing built-in).
Take it slow.

## The problem async solves

A game server connection spends almost all its time **waiting** — for the next packet from the
client. With OS threads (module 10), one blocked thread per connection wastes memory and scheduler
time; a few thousand connections and you're in trouble.

**Async** lets a small number of OS threads juggle a huge number of tasks: when a task is waiting
on the network, the thread sets it aside and runs another ready task. Idle connections cost almost
nothing.

- **Python** `asyncio`: `async def` / `await` / event loop. Same idea.
- **Rust**: `async fn` / `.await`, but you bring your own runtime (Tokio).

## async / await basics

```rust
async fn fetch_player(id: u32) -> Player {   // returns a Future, not a Player (yet)
    // ...
}

async fn handler() {
    let p = fetch_player(42).await;   // .await drives the future to completion, yielding meanwhile
    println!("{}", p.name);
}
```

Key facts that surprise newcomers:

1. **`async fn` returns a `Future` immediately and runs nothing.** A Future is a lazy computation.
2. **Nothing happens until you `.await` it** (or hand it to the runtime). Calling an async fn
   without awaiting does *zero* work — different from Python where the coroutine at least exists.
3. **`.await` is a *yield point*:** "if this isn't ready, let the runtime run something else." It's
   cooperative — a task only yields at `.await`, so don't block (no `std::thread::sleep`, no heavy
   CPU loops) inside async code, or you stall everything on that thread.

## You need a runtime: Tokio

Futures don't run themselves; a runtime (executor) polls them. Tokio is the standard.

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]                      // macro: sets up the runtime and runs main as a task
async fn main() {
    println!("server starting");
    do_async_thing().await;
}
```

`#[tokio::main]` expands to creating a Tokio runtime and blocking on your async `main`. That's your
entry point.

## Spawning async tasks

The async analog of `thread::spawn`. Tasks are cheap — you can have hundreds of thousands.

```rust
let handle = tokio::spawn(async move {
    handle_connection(socket).await;
});
// handle.await to wait for it (returns a Result)
```

Typical server shape: accept a connection, `tokio::spawn` a task to handle it, loop back to accept
the next. Each connection is one lightweight task.

## Async channels and shared state

Tokio provides async versions of the concurrency primitives from module 10:

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<ClientMessage>(100);   // bounded channel

// producer task:
tx.send(msg).await.unwrap();

// consumer (e.g. the game loop):
while let Some(msg) = rx.recv().await {
    process(msg);
}
```

For shared mutable state in async code, use **`tokio::sync::Mutex`** (not `std::sync::Mutex`),
because it yields instead of blocking the thread while waiting:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let world = Arc::new(Mutex::new(World::new()));
let w = Arc::clone(&world);
tokio::spawn(async move {
    let mut guard = w.lock().await;   // .await, not .lock().unwrap()
    guard.tick();
});
```

Other Tokio sync tools you'll use: `broadcast` (send one message to many subscribers — great for
"player X moved" world updates), `oneshot` (single reply), `RwLock`.

## select! — wait on multiple things at once

A connection task usually needs to do two things concurrently: read from the socket AND receive
server-pushed updates to send to the client. `tokio::select!` waits on whichever is ready first:

```rust
loop {
    tokio::select! {
        result = socket.read(&mut buf) => {
            // client sent us something
        }
        Some(update) = updates_rx.recv() => {
            // server has an update to push to this client
            socket.write_all(&update).await?;
        }
    }
}
```

This is the heart of a connection handler. (Python's `asyncio` equivalent is `asyncio.wait` with
FIRST_COMPLETED, but `select!` is more ergonomic.)

## Timers and the tick loop

```rust
use tokio::time::{interval, Duration};

let mut ticker = interval(Duration::from_millis(50));   // 20 ticks/second
loop {
    ticker.tick().await;        // wait until next tick — non-blocking
    world.lock().await.update(); // advance the game one step
    // broadcast new state to players
}
```

A fixed-rate tick loop like this is the backbone of a game server: each tick, apply queued player
actions, run game logic (movement, combat, AI), and push the new state to clients.

## Mental model summary

| Concept | Threads (mod 10) | Async (this module) |
|---------|------------------|---------------------|
| Unit of work | OS thread | task (Future) |
| Cost each | ~MBs, OS-scheduled | ~KBs, runtime-scheduled |
| Good for | CPU-bound work | many I/O-bound connections |
| Spawn | `thread::spawn` | `tokio::spawn` |
| Shared state | `Arc<std::Mutex>` | `Arc<tokio::Mutex>` |
| Block? | blocking is fine | NEVER block; use `.await` |

For an MMORPG: **async (Tokio) for all the networking and per-connection tasks**, plus threads if
you have heavy CPU work (physics, pathfinding) you want off the async runtime (`tokio::task::
spawn_blocking` bridges the two).

## Gotchas to expect

- "future is not `Send`" errors when spawning — usually means you held a non-Send type (like a
  `std::sync::MutexGuard` or `Rc`) across an `.await`. Drop it before awaiting, or use Tokio's
  types.
- Forgetting `.await` → the code compiles but does nothing (compiler warns: "unused Future").
- Blocking calls inside async (file I/O via `std::fs`, `thread::sleep`, big loops) freeze the
  runtime thread. Use Tokio's async equivalents or `spawn_blocking`.

---

🏋️ Do `Exercises/11_async.md`.

➡️ Next: `12_networking.md`
