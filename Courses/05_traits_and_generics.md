# 05 — Traits & Generics

Traits are Rust's answer to interfaces / abstract base classes / C++ concepts. Generics are like
C++ templates but checked up front. Together they're how you write reusable code for many entity
types without inheritance (Rust has **no class inheritance** — and you won't miss it).

## Traits = shared behavior (interfaces)

A trait defines methods a type must provide:

```rust
trait Damageable {
    fn take_damage(&mut self, amount: u32);
    fn is_alive(&self) -> bool;

    // traits can provide DEFAULT implementations:
    fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}
```

Implement it for your types:

```rust
impl Damageable for Player {
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
    fn is_alive(&self) -> bool { self.hp > 0 }
    // is_dead() comes free from the default
}

impl Damageable for Monster {
    fn take_damage(&mut self, amount: u32) { self.hp = self.hp.saturating_sub(amount); }
    fn is_alive(&self) -> bool { self.hp > 0 }
}
```

Now any code that needs "something damageable" works with both, with no shared base class.

- **Python**: duck typing — works if the method exists, discovered at runtime.
- **C**: no real equivalent; you fake it with function pointers in structs.
- **Rust**: the contract is explicit and checked at compile time. Best of both.

## Using traits: generics vs trait objects

### Static dispatch (generics) — fast, monomorphized

```rust
fn attack<T: Damageable>(target: &mut T, dmg: u32) {
    target.take_damage(dmg);
}
// or the equivalent `impl Trait` shorthand:
fn attack2(target: &mut impl Damageable, dmg: u32) { target.take_damage(dmg); }
```

`<T: Damageable>` is a **trait bound**: "T can be any type that implements Damageable." The
compiler generates a specialized version per concrete type (like C++ templates). Zero runtime cost.

### Dynamic dispatch (trait objects) — flexible, runtime

When you want a collection of *different* types behind one trait:

```rust
let mut entities: Vec<Box<dyn Damageable>> = vec![
    Box::new(player),
    Box::new(monster),
];

for e in entities.iter_mut() {
    e.take_damage(5);     // virtual call, resolved at runtime (like a vtable in C++)
}
```

`dyn Damageable` = "some type implementing Damageable, decided at runtime." Slight overhead
(a pointer indirection), but lets you mix types in one collection. You'll use both styles in a
game; ECS (module 14) often avoids `dyn` for speed.

## Generic data structures

```rust
struct Inventory<T> {
    items: Vec<T>,
}

impl<T> Inventory<T> {
    fn new() -> Self { Self { items: Vec::new() } }
    fn add(&mut self, item: T) { self.items.push(item); }
}
```

Just like `Vec<T>`, `Option<T>`, `HashMap<K, V>` — all generic. You can constrain the generics:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &x in list { if x > max { max = x; } }
    max
}
```

`T: PartialOrd + Copy` = "T must be comparable AND copyable." Multiple bounds with `+`.

## The standard traits you'll meet constantly

| Trait | Gives you | How to get it |
|-------|-----------|---------------|
| `Debug` | `{:?}` printing | `#[derive(Debug)]` |
| `Clone` | `.clone()` | `#[derive(Clone)]` |
| `Copy` | implicit copy (small types) | `#[derive(Copy, Clone)]` |
| `PartialEq` / `Eq` | `==` | `#[derive(PartialEq)]` |
| `PartialOrd` / `Ord` | `< > <= >=`, sorting | `#[derive(PartialOrd, Ord)]` |
| `Hash` | use as HashMap key | `#[derive(Hash)]` |
| `Default` | `Type::default()` | `#[derive(Default)]` |
| `Display` | `{}` user-facing print | implement by hand |
| `Iterator` | `for` loops, `.map()`… | implement `next()` |
| `From` / `Into` | conversions, powers `?` | implement `From` |

Most of the time you just `#[derive(...)]` them.

### Implementing Display

`Debug` is for developers; `Display` is the user-facing `{}` format you write yourself:

```rust
use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (lvl {}, {} hp)", self.name, self.level, self.hp)
    }
}

println!("{p}");   // uses Display
```

### From/Into and the `?` connection

Implement `From` and you get `.into()` for free, plus `?` can auto-convert errors:

```rust
impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> Self { GameError::Io(e) }
}
// now `something_io()?` inside a fn returning Result<_, GameError> just works
```

## Why no inheritance?

Rust deliberately omits class inheritance. Instead:
- **Share behavior** → traits (with default methods).
- **Share data/structure** → composition (put a struct inside another struct).

For games this is liberating: instead of a brittle `Entity → Creature → Humanoid → Player` tree,
you compose capabilities (`Health`, `Position`, `Inventory`) and implement traits. This is exactly
the philosophy behind ECS (module 14).

## Associated types & generic traits (preview)

Some traits have an associated type, most famously `Iterator`:

```rust
trait Iterator {
    type Item;                       // associated type
    fn next(&mut self) -> Option<Self::Item>;
}
```

You'll lean on this in module 06. Implementing `Iterator` for your own types lets them work with
`for`, `.map()`, `.filter()`, etc.

---

🏋️ Do `Exercises/05_traits_generics.md`.

➡️ Next: `06_collections_and_iterators.md`
