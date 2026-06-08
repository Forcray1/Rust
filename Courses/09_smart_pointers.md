# 09 — Smart Pointers: Box, Rc, Arc, RefCell

Sometimes the simple "one owner" model isn't enough: you need heap allocation, shared ownership,
or mutation through a shared reference. Smart pointers give you those — each with a clear cost and
purpose. For a multiplayer server, `Arc` + a lock is the key combo.

## Box<T> — owned heap allocation

```rust
let b: Box<i32> = Box::new(5);     // value lives on the heap, b owns it
```

Use Box when:
1. **A type must have a known size but contains itself (recursion).** Classic: a tree/linked list.

```rust
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),    // without Box this would be infinitely sized
}
```

2. **Trait objects:** `Box<dyn Trait>` (from module 05) — store different types behind one trait.
3. **Move a large value without copying it** (just moves a pointer).

Box has single ownership, like a normal value — just heap-placed. Cheap, no runtime tracking.

## Rc<T> — shared ownership, single-threaded

`Rc` = "reference counted." Lets **multiple owners** share one value; the data is freed when the
last `Rc` is dropped. This is essentially Python's object model, but explicit and only when asked.

```rust
use std::rc::Rc;

let shared = Rc::new(Player::new("Aria".into()));
let a = Rc::clone(&shared);   // cheap: bumps the count, does NOT deep-copy the Player
let b = Rc::clone(&shared);   // 3 owners now
println!("{}", Rc::strong_count(&shared));   // 3
```

`Rc` is **single-threaded only** (its counter isn't thread-safe). For multithreading you use `Arc`.

## Arc<T> — shared ownership, thread-safe

`Arc` = "atomic reference counted." Same as `Rc` but safe to share across threads. This is the one
you'll use in a server.

```rust
use std::sync::Arc;

let world = Arc::new(World::new());
let w2 = Arc::clone(&world);     // give a clone to another thread/task
std::thread::spawn(move || {
    println!("{}", w2.player_count());
});
```

But `Arc<T>` only gives **shared read** access — you can't mutate through it (borrow rules!). To
mutate shared data, combine it with a lock.

## Interior mutability: mutate through a shared reference

The borrow rules say "no mutation while shared." Sometimes you genuinely need shared *and* mutable.
The escape hatch is **interior mutability** — the borrow check moves from compile time to runtime.

### RefCell<T> — single-threaded interior mutability

```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() += 1;          // runtime-checked mutable borrow
println!("{}", cell.borrow());    // 6
```

If you violate the rules (e.g., two `borrow_mut()` at once), it **panics at runtime** instead of
failing to compile. Common pairing: `Rc<RefCell<T>>` = shared, mutable, single-threaded.

### Mutex / RwLock — thread-safe interior mutability

For shared mutable state across threads, wrap in `Arc` and a lock:

```rust
use std::sync::{Arc, Mutex};

let world = Arc::new(Mutex::new(World::new()));

let w = Arc::clone(&world);
std::thread::spawn(move || {
    let mut guard = w.lock().unwrap();   // blocks until the lock is free
    guard.spawn_monster();
});  // guard dropped here → lock released
```

- **`Mutex<T>`** — one accessor at a time (read or write).
- **`RwLock<T>`** — many readers OR one writer (better when reads dominate, common in games).

`Arc<Mutex<T>>` is *the* canonical "shared mutable game state across threads" pattern. (For async
code you'll use `tokio::sync::Mutex` instead — module 11.)

## The decision table

| You need… | Use |
|-----------|-----|
| Heap allocation, single owner | `Box<T>` |
| Recursive type | `Box<T>` |
| Many types behind a trait, in a collection | `Box<dyn Trait>` |
| Multiple owners, single thread | `Rc<T>` |
| Multiple owners, multiple threads | `Arc<T>` |
| Mutate shared data, single thread | `Rc<RefCell<T>>` |
| Mutate shared data, multiple threads | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

## Costs (so you choose wisely in a game loop)

- `Box`: one allocation, then free as a normal value. Negligible.
- `Rc`/`Arc`: a small counter bump on clone/drop. `Arc` uses atomic ops (slightly pricier). Cloning
  an `Arc` does **not** copy the data — only the pointer + count, so it's cheap.
- `RefCell`: a runtime borrow-flag check; can panic. Tiny cost.
- `Mutex`/`RwLock`: locking has real cost and can cause contention if many threads fight over it.
  In a server you'll think about lock *granularity* — one big `Mutex<World>` is simple but can
  bottleneck; finer-grained locks scale better but are more complex. Start simple.

## A glimpse of cyclic references

`Rc`/`Arc` can leak memory if you make a cycle (A owns B owns A). The fix is `Weak<T>` — a
non-owning reference that doesn't keep data alive. You'll meet this if you build, say, a scene
graph where children point back to parents. Note it exists; reach for it only when needed.

---

🏋️ Do `Exercises/09_smart_pointers.md`.

➡️ Next: `10_concurrency.md` ⭐
