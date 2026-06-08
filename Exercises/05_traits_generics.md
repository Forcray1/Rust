# Exercises 05 — Traits & Generics

Reusable behavior without inheritance — the foundation for game entities.

## Drills

**D1.** Define `trait Describe { fn describe(&self) -> String; }`. Implement it for `Player` and a
`Monster` struct. Write `fn announce(thing: &impl Describe)` that prints the description. Call it
with both types.

**D2.** Add a default method to `Describe`: `fn shout(&self) -> String { self.describe().to_uppercase() }`.
Override it for `Monster` only. Confirm `Player` uses the default.

**D3.** Implement `std::fmt::Display` for `Player` so `println!("{p}")` prints
`"Aria (lvl 3, 80hp)"`.

**D4.** Write a generic `fn max_by_key<T, K: PartialOrd>(items: &[T], key: impl Fn(&T) -> K) -> Option<&T>`
that returns the element with the largest key. Test: highest-level player in a slice.

## Mini-project: Damageable + a generic container

Two parts that you'll reuse in the game.

**Part A — the trait.**
```rust
trait Damageable {
    fn take_damage(&mut self, amount: u32);
    fn hp(&self) -> u32;
    fn is_alive(&self) -> bool { self.hp() > 0 }   // default method
}
```
Implement it for `Player` and `Monster`. Then write a function that works on *any* mix of
damageable things via dynamic dispatch:
```rust
fn aoe_blast(targets: &mut [Box<dyn Damageable>], dmg: u32) {
    // damage everything, then report how many survived
}
```

**Part B — a generic inventory.**
```rust
struct Inventory<T> { items: Vec<T> }

impl<T> Inventory<T> {
    fn new() -> Self { /* ... */ }
    fn add(&mut self, item: T) { /* ... */ }
    fn count(&self) -> usize { /* ... */ }
}
```
Make it work with `Inventory<String>` and `Inventory<u32>`. Add a method
`fn find<F: Fn(&T) -> bool>(&self, pred: F) -> Option<&T>`.

**Done when:** `aoe_blast` damages a heterogeneous `Vec<Box<dyn Damageable>>` and reports
survivors, and `Inventory<T>` works for at least two element types.

---
<details><summary>Solution sketch</summary>

```rust
fn aoe_blast(targets: &mut [Box<dyn Damageable>], dmg: u32) {
    for t in targets.iter_mut() { t.take_damage(dmg); }
    let alive = targets.iter().filter(|t| t.is_alive()).count();
    println!("{alive} survivors");
}

impl<T> Inventory<T> {
    fn new() -> Self { Self { items: Vec::new() } }
    fn add(&mut self, item: T) { self.items.push(item); }
    fn count(&self) -> usize { self.items.len() }
    fn find<F: Fn(&T) -> bool>(&self, pred: F) -> Option<&T> {
        self.items.iter().find(|x| pred(x))
    }
}
```
</details>
