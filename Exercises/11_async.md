# Exercises 11 — Async & Tokio

New mental model. Go slowly; run everything.

Setup: `cargo add tokio --features full`

## Drills

**D1. Hello async.**
```rust
#[tokio::main]
async fn main() {
    println!("start");
    say_hi().await;
    println!("end");
}
async fn say_hi() { println!("hi"); }
```
Then **remove** the `.await` on `say_hi()` and observe: the compiler warns the future is unused and
"hi" never prints. Lesson: async fns do nothing until awaited.

**D2. Concurrent sleeps.** Spawn three tasks that each `tokio::time::sleep` for 1, 2, 3 seconds and
print when done. Await all three. Total wall-clock should be ~3s (concurrent), not 6s (sequential).
```rust
use tokio::time::{sleep, Duration};
let a = tokio::spawn(async { sleep(Duration::from_secs(1)).await; println!("a"); });
// ... b, c ... then a.await, b.await, c.await
```

**D3. join! vs sequential.** Use `tokio::join!(task1(), task2())` to run two async fns
concurrently and collect both results. Compare to awaiting them one-by-one.

## Mini-project A: a channel + consumer

Mirror the channel architecture from module 10, but async.

```rust
use tokio::sync::mpsc;

#[derive(Debug)]
enum Event { Join(u32), Chat(u32, String), Quit }

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Event>(32);

    // producer tasks:
    for id in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(Event::Join(id)).await.unwrap();
            tx.send(Event::Chat(id, format!("hi from {id}"))).await.unwrap();
        });
    }
    drop(tx);

    // consumer (your future "game loop"):
    while let Some(event) = rx.recv().await {
        match event {
            Event::Join(id) => println!("player {id} joined"),
            Event::Chat(id, msg) => println!("[{id}] {msg}"),
            Event::Quit => break,
        }
    }
}
```

## Mini-project B: a tick loop with select!

Build the skeleton of a game loop that does two things at once: tick on an interval AND react to
events.

```rust
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // simulate input arriving every 700ms
    tokio::spawn(async move {
        let mut i = 0;
        loop {
            tokio::time::sleep(Duration::from_millis(700)).await;
            if tx.send(format!("action {i}")).await.is_err() { break; }
            i += 1;
        }
    });

    let mut ticker = interval(Duration::from_millis(500));
    let mut tick = 0u32;
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                tick += 1;
                println!("-- tick {tick} --");
                if tick >= 10 { break; }     // stop after 10 ticks
            }
            Some(action) = rx.recv() => {
                println!("got: {action}");
            }
        }
    }
}
```

**Done when:** you see ticks firing at ~500ms intervals *interleaved* with actions at ~700ms — one
loop handling both. This `select!`-in-a-loop is the literal heart of your server's game loop and
per-connection handlers.

---
<details><summary>Hints</summary>

- `#[tokio::main]` is required on `main` to start the runtime.
- Never call `std::thread::sleep` in async code — use `tokio::time::sleep`. Blocking the thread
  freezes other tasks.
- If `tokio::spawn` complains "future is not Send", you held a non-Send value (e.g. `Rc`) across an
  `.await`. Use owned/Send types.
</details>
