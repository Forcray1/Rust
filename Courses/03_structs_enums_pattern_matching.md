# 03 — Structs, Enums & Pattern Matching

This is how you model your game's data. Enums + pattern matching are far more powerful than
anything in C, and the `match` expression will become your favorite tool.

## Structs

Like C structs / Python classes-without-methods:

```rust
struct Player {
    name: String,
    hp: u32,
    level: u8,
    position: (f32, f32),
}

let p = Player {
    name: String::from("Aria"),
    hp: 100,
    level: 1,
    position: (0.0, 0.0),
};

println!("{} has {} hp", p.name, p.hp);
```

Other forms:

```rust
struct Point(f32, f32);      // tuple struct — fields by index: p.0, p.1
struct Marker;               // unit struct — no data (useful as a type tag)
```

### Methods via `impl`

Rust separates data (`struct`) from behavior (`impl`), unlike Python where they're in one block:

```rust
impl Player {
    // associated function (no self) — like a static method / constructor
    fn new(name: String) -> Self {
        Self { name, hp: 100, level: 1, position: (0.0, 0.0) }
    }

    // method — borrows self immutably
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    // method — borrows self mutably (can change fields)
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);   // saturating = won't underflow below 0
    }

    // method that consumes self (takes ownership) — rarer
    fn into_corpse(self) -> Corpse { /* ... */ }
}

let mut p = Player::new(String::from("Aria"));   // :: for associated functions
p.take_damage(30);                               // . for methods
println!("{}", p.is_alive());
```

`self`, `&self`, `&mut self` map exactly onto the ownership rules from module 02:
- `&self` — read-only method (most common)
- `&mut self` — mutating method
- `self` — consumes the object

### Field init shorthand

If a variable has the same name as the field, write it once: `Self { name, hp: 100, .. }`.
The `..` in struct *construction* doesn't exist, but `..other` copies remaining fields:

```rust
let p2 = Player { name: String::from("Bob"), ..p };   // rest of fields from p
```

## Enums — the superpower

C enums are just named integers. Rust enums are **sum types**: each variant can carry *different
data*. This is the single most useful modeling tool in the language.

```rust
enum Entity {
    Player { name: String, hp: u32 },   // struct-like variant
    Monster(String, u32),               // tuple-like variant (kind, hp)
    Item(u32),                          // carries an item id
    Empty,                              // no data
}
```

A value of type `Entity` is *exactly one* of these. This is perfect for:

- **Network packets** — one enum, one variant per message type
- **Game states** — `Lobby`, `InGame`, `Paused`, `GameOver`
- **Tile/cell contents**, **abilities**, **AI states**, etc.

```rust
// The packet enum you'll actually build later:
enum ClientMessage {
    Login { username: String },
    Move { dx: f32, dy: f32 },
    Chat(String),
    Attack { target_id: u32 },
    Logout,
}
```

### `Option` and `Result` are just enums

The standard library's two most important types are ordinary enums (you'll use them constantly):

```rust
enum Option<T> { Some(T), None }          // a value, or nothing (replaces null)
enum Result<T, E> { Ok(T), Err(E) }       // success, or an error (replaces exceptions)
```

Full treatment in module 04.

## Pattern matching with `match`

`match` is like `switch` on steroids — it destructures data and the compiler forces you to handle
**every** case (exhaustiveness):

```rust
fn describe(e: &Entity) -> String {
    match e {
        Entity::Player { name, hp } => format!("Player {name} with {hp} hp"),
        Entity::Monster(kind, hp)   => format!("A {kind} ({hp} hp)"),
        Entity::Item(id)            => format!("Item #{id}"),
        Entity::Empty               => String::from("nothing"),
    }
}
```

If you forget a variant, **it won't compile.** Add a new variant to the enum later, and the
compiler shows you *every* `match` you need to update. This is huge for maintaining a big game.

### Match is an expression

It returns a value, so you assign from it:

```rust
let speed = match terrain {
    Terrain::Road  => 10,
    Terrain::Grass => 6,
    Terrain::Swamp => 3,
    _ => 5,                 // `_` = catch-all / default
};
```

### Match guards and ranges

```rust
match hp {
    0 => "dead",
    1..=20 => "critical",
    21..=50 => "wounded",
    _ => "healthy",
}

match point {
    (0, 0) => "origin",
    (x, 0) => "on x-axis",          // binds x
    (x, y) if x == y => "diagonal", // guard
    _ => "elsewhere",
}
```

## `if let` and `let else` — match for one case

When you only care about one pattern, `match` is verbose. Use `if let`:

```rust
if let Entity::Player { name, .. } = &entity {
    println!("It's a player named {name}");
}
```

`while let` loops while a pattern matches (great for draining a queue):

```rust
while let Some(packet) = queue.pop() {
    handle(packet);
}
```

`let ... else` for the "extract or bail" pattern:

```rust
let Some(player) = world.get(id) else {
    return;   // not found → leave early
};
// player is available here
```

## Deriving common behavior

Add `#[derive(...)]` above a type to auto-generate trait implementations:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Position { x: f32, y: f32 }
```

- `Debug` → printable with `{:?}` (always derive this while learning)
- `Clone` → `.clone()` works
- `Copy` → make it a Copy type (only for small all-Copy fields)
- `PartialEq` → `==` works
- `Default` → `Type::default()`

Traits (what derive implements) are module 05.

---

🏋️ Do `Exercises/03_structs_enums.md`.

➡️ Next: `04_error_handling.md`
