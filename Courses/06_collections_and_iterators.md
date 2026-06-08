# 06 — Collections & Iterators

Where your world state lives, and the idiomatic way to process it. Iterators are how Rust does the
work Python does with list comprehensions and `for` loops — but with zero overhead.

## The core collections

```rust
use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};

let mut v: Vec<i32> = vec![1, 2, 3];          // growable array (Python list / C++ vector)
let mut map: HashMap<u32, Player> = HashMap::new();  // dict; great for id -> entity
let mut set: HashSet<u32> = HashSet::new();   // unique values
let mut queue: VecDeque<Packet> = VecDeque::new();   // double-ended queue (message buffers)
let mut sorted: BTreeMap<u32, String> = BTreeMap::new(); // sorted-by-key map
```

### Vec — the workhorse

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
let first = v[0];            // panics if out of bounds
let maybe = v.get(5);        // returns Option<&i32> — safe
v.len(); v.is_empty();
v.pop();                     // Option<T>
for x in &v { }              // borrow each element
for x in &mut v { *x += 1; } // mutate each
```

### HashMap — your entity registry

```rust
let mut players: HashMap<u32, Player> = HashMap::new();
players.insert(1, player_a);
players.insert(2, player_b);

if let Some(p) = players.get(&1) { /* &Player */ }
if let Some(p) = players.get_mut(&1) { p.hp -= 10; }   // mutable access

// "get or insert" pattern — extremely common:
let counter = players.entry(3).or_insert_with(|| Player::new("Carol".into()));

players.remove(&2);          // player disconnects
for (id, p) in &players { }  // iterate entries
```

> In an MMORPG, `HashMap<EntityId, Entity>` (or an ECS) is your live world. `get_mut` + the borrow
> rules from module 02 are why you can't accidentally alias two mutable references to one entity.

## Iterators — the idiomatic Rust

Instead of manual index loops, you chain *adapters*. This reads like Python comprehensions /
functional style, but compiles to code as fast as a hand-written loop ("zero-cost abstraction").

```rust
let nums = vec![1, 2, 3, 4, 5, 6];

let evens_doubled: Vec<i32> = nums
    .iter()                       // &i32 over the vec
    .filter(|&&x| x % 2 == 0)     // keep evens
    .map(|&x| x * 2)              // double them
    .collect();                   // gather into a Vec  -> [4, 8, 12]
```

Python equivalent: `[x*2 for x in nums if x%2==0]`. Same idea, statically typed and as fast as C.

### iter / iter_mut / into_iter (the ownership angle)

| Method | Yields | Use when |
|--------|--------|----------|
| `.iter()` | `&T` | read-only pass |
| `.iter_mut()` | `&mut T` | modify in place |
| `.into_iter()` | `T` (consumes the collection) | you're done with it / want ownership |

This maps straight onto module 02. The borrow rules apply: you can't mutate the collection's
structure while an iterator borrows it.

### The adapters you'll use most

```rust
.map(|x| ...)                  // transform each
.filter(|x| ...)               // keep matching
.filter_map(|x| ...)           // map + drop None in one pass
.find(|x| ...)                 // first match -> Option
.any(|x| ...) / .all(|x| ...)  // bool checks
.count()                       // how many
.sum() / .max() / .min()       // reductions
.enumerate()                   // (index, item) pairs
.zip(other)                    // pair up two iterators
.take(n) / .skip(n)
.fold(init, |acc, x| ...)      // general reduction (like Python's reduce)
.collect()                     // materialize into Vec / HashMap / String...
.for_each(|x| ...)             // side effects (or just use a for loop)
```

### Lazy and chainable

Iterator adapters are **lazy** — nothing runs until a "consumer" (`collect`, `sum`, `for`, `count`,
`find`…) drives them. So chaining many adapters does *one* pass, not one pass per adapter.

```rust
// Find living players within range — one pass, no intermediate allocations:
let nearby: Vec<&Player> = players
    .values()
    .filter(|p| p.is_alive())
    .filter(|p| distance(p.pos, center) < radius)
    .collect();
```

### collect() into different types

```rust
let v: Vec<_>            = iter.collect();
let s: String           = chars.collect();
let m: HashMap<_, _>     = pairs.collect();         // from (k, v) tuples
let r: Result<Vec<_>, _> = results.collect();       // short-circuits on first Err — very handy
```

That last one is a gem: collecting an iterator of `Result`s into `Result<Vec<_>, E>` stops at the
first error. Parse a whole packet's worth of fields and bail cleanly if any is malformed.

## Closures (the `|x| ...` things)

Closures are anonymous functions that can capture their environment — like Python lambdas but
unrestricted (multi-line, can mutate captures):

```rust
let factor = 3;
let scale = |x: i32| x * factor;     // captures `factor`
println!("{}", scale(10));           // 30
```

Three capture modes mirror ownership: by `&` (read), by `&mut` (mutate), or by value (`move`
keyword — needed when passing a closure to another thread, module 10):

```rust
let data = vec![1, 2, 3];
let f = move || println!("{:?}", data);   // takes ownership of `data`
```

## A taste of how this looks in a game tick

```rust
// Apply regen to all living players, collect the dead for cleanup:
let dead: Vec<u32> = players
    .iter_mut()
    .map(|(id, p)| { p.hp = (p.hp + 1).min(p.max_hp); (*id, p.is_alive()) })
    .filter(|(_, alive)| !alive)
    .map(|(id, _)| id)
    .collect();

for id in dead {
    players.remove(&id);
}
```

This is the shape of real game-loop code: iterate the world, update, collect changes, apply them.

---

🏋️ Do `Exercises/06_collections_iterators.md`.

➡️ Next: `07_lifetimes.md`
