# Exercises 09 — Smart Pointers

Shared ownership and shared mutation — the groundwork for shared game state.

## Drills

**D1. Box for recursion.** Define and build a singly linked list of i32 using
`enum List { Cons(i32, Box<List>), Nil }`. Write `fn sum(list: &List) -> i32` recursively.

**D2. Rc sharing.** Create an `Rc<String>`, clone it twice, and print `Rc::strong_count`. Confirm
all three handles see the same data and the count is 3.

**D3. Rc<RefCell<T>>.** Make a shared, mutable counter:
```rust
use std::rc::Rc;
use std::cell::RefCell;
let counter = Rc::new(RefCell::new(0));
let a = Rc::clone(&counter);
*a.borrow_mut() += 1;
*counter.borrow_mut() += 1;
println!("{}", counter.borrow());   // 2
```
Then deliberately trigger a panic by holding a `borrow()` and a `borrow_mut()` at the same time —
observe the *runtime* borrow check.

## Mini-project: from shared counter to shared world

**Part A — Arc<Mutex<i32>>.** Adapt D3 to be thread-safe (preview of module 10): wrap an `i32` in
`Arc<Mutex<>>`, but for now just use it from `main` (single thread):
```rust
use std::sync::{Arc, Mutex};
let score = Arc::new(Mutex::new(0));
let handle = Arc::clone(&score);
*handle.lock().unwrap() += 10;
println!("{}", *score.lock().unwrap());   // 10
```

**Part B — a shared world handle.** Build the data structure your server will share:
```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug)]
struct Player { name: String, hp: u32 }

struct World { players: HashMap<u32, Player> }

impl World {
    fn new() -> Self { Self { players: HashMap::new() } }
    fn add(&mut self, id: u32, name: &str) {
        self.players.insert(id, Player { name: name.into(), hp: 100 });
    }
    fn count(&self) -> usize { self.players.len() }
}

fn main() {
    let world = Arc::new(Mutex::new(World::new()));

    // Simulate two "connections" adding players (still single-threaded here):
    let w1 = Arc::clone(&world);
    w1.lock().unwrap().add(1, "Aria");

    let w2 = Arc::clone(&world);
    w2.lock().unwrap().add(2, "Bob");

    println!("players online: {}", world.lock().unwrap().count());   // 2
}
```

**Done when:** both parts compile and print the expected values. You now hold the exact pattern
(`Arc<Mutex<World>>`) used to share game state — module 10 will make multiple threads hit it for
real.

---
<details><summary>Solution sketch</summary>

D1:
```rust
enum List { Cons(i32, Box<List>), Nil }
fn sum(list: &List) -> i32 {
    match list {
        List::Cons(v, rest) => v + sum(rest),
        List::Nil => 0,
    }
}
// build: List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
```
The mini-project code above is already complete — type it out, run it, then change `add` to take
`&mut self` vs `self` and observe what breaks. That contrast is the lesson.
</details>
