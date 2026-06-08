# 10 — Concurrency: Threads & Channels ⭐

A multiplayer server is fundamentally a concurrency problem: many players act at once. Rust's
headline feature is **"fearless concurrency"** — the same ownership/borrow rules that prevent
memory bugs *also* prevent data races, at compile time. The bug class that plagues C/C++ threading
simply doesn't compile.

## Spawning threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    for i in 0..5 {
        println!("from thread: {i}");
    }
});

handle.join().unwrap();   // wait for the thread to finish
```

`thread::spawn` takes a closure and runs it on a new OS thread. `join()` waits for it.

## Moving data into threads

A thread may outlive the current scope, so it can't *borrow* local data — it must **own** what it
uses. Hence `move`:

```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {       // `move` transfers ownership of `data` into the thread
    println!("{:?}", data);
});
handle.join().unwrap();
// `data` is no longer usable here — it was moved
```

This is the `'static` requirement from module 07 in action. The compiler guarantees the thread
can't reference freed stack data — no use-after-free across threads.

## Sharing state across threads: Arc + Mutex

To let multiple threads touch the *same* data, share ownership with `Arc` and guard mutation with a
`Mutex` (module 09):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);          // each thread gets its own Arc handle
    handles.push(thread::spawn(move || {
        let mut n = c.lock().unwrap();      // acquire lock
        *n += 1;
    }));                                    // lock released when `n` is dropped
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());   // 10 — guaranteed, no race
```

In C this exact program is a minefield (forget the lock → torn writes). In Rust, **you cannot
access the inner value without locking** — the data is *inside* the Mutex, so the type system makes
the unsafe version impossible to write.

## Send and Sync — why it's "fearless"

Two marker traits the compiler checks automatically:

- **`Send`**: a type is safe to *move* to another thread.
- **`Sync`**: a type is safe to *share* (`&T`) across threads.

`Arc<Mutex<T>>` is Send+Sync; plain `Rc`/`RefCell` are **not** (that's why they're single-thread
only). If you try to send a non-Send type to a thread, it won't compile. You rarely think about
these directly — but they're the machinery that catches data races for you.

## Channels — message passing (often better than shared state)

Instead of sharing memory and locking, you can pass *messages* between threads. This is frequently
the cleaner architecture for a game server: each connection sends events to a central game thread.

```rust
use std::sync::mpsc;     // multi-producer, single-consumer
use std::thread;

let (tx, rx) = mpsc::channel();

// Several producer threads:
for id in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(format!("player {id} joined")).unwrap();
    });
}
drop(tx);    // drop the original so the channel closes when all senders are gone

// One consumer:
for msg in rx {          // iterates until all senders are dropped
    println!("received: {msg}");
}
```

> "Do not communicate by sharing memory; share memory by communicating." Channels move ownership of
> the message to the receiver — no locks, no races by construction.

## The two classic server architectures

### A) Shared world + locks
```
[conn thread 1] ─┐
[conn thread 2] ─┼──> Arc<Mutex<World>>   (everyone locks to read/write)
[conn thread 3] ─┘
```
Simple to start. Risk: lock contention as players grow.

### B) Message passing to a single game thread (recommended)
```
[conn thread 1] ──tx──┐
[conn thread 2] ──tx──┼──> [game thread]  owns World exclusively, no locks
[conn thread 3] ──tx──┘     processes a queue of events each tick,
                             sends updates back via per-player channels
```
The game thread is the *only* owner of `World`, so no locking is needed for it. Connections talk to
it via channels. This scales better and is easier to reason about. The "tick loop" lives here.

## Threads vs async (the cliffhanger)

OS threads are great, but you can't have 50,000 of them (each costs memory + the OS scheduler).
For an MMORPG with thousands of mostly-idle connections (waiting on the network), you want **async
tasks** — millions of cheap "green" tasks multiplexed onto a few threads. That's the next module,
and it's how real network servers are built. You'll still use threads for CPU-bound work and
channels everywhere.

## Key takeaways

- `thread::spawn(move || ...)` + `join()`.
- Share state: `Arc<Mutex<T>>` / `Arc<RwLock<T>>`. The data lives inside the lock.
- Prefer **channels** (message passing) for game-server architecture — fewer locks, clearer flow.
- `Send`/`Sync` are checked by the compiler: data races don't compile.

---

🏋️ Do `Exercises/10_concurrency.md`.

➡️ Next: `11_async_and_tokio.md` ⭐
