# Exercises 06 — Collections & Iterators

Where world state lives and how you process it each tick.

## Drills

Given `let nums = vec![4, 8, 15, 16, 23, 42];`, write each as a single iterator chain:

**D1.** Sum of the even numbers.
**D2.** A `Vec<i32>` of each number squared.
**D3.** The first number greater than 20 (`Option<&i32>`).
**D4.** A `Vec<(usize, i32)>` of (index, value) for values > 10 (use `.enumerate()` + `.filter()`).
**D5.** Do all numbers fit in a `u8`? (`.all(|&x| x <= 255)`)
**D6.** Build a `HashMap<i32, i32>` mapping each number to its square (`.map(...).collect()`).

## Drill on HashMap

**D7.** Count word frequency in `"the cat sat on the mat the end"` → `HashMap<&str, u32>` using the
`entry(...).or_insert(0)` pattern. Print the count for `"the"` (should be 3).

## Mini-project: a tiny inventory & query engine

Model a player registry and run the kinds of queries a game tick needs.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Player {
    id: u32,
    name: String,
    hp: u32,
    pos: (f32, f32),
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn main() {
    let mut world: HashMap<u32, Player> = HashMap::new();
    // TODO: insert ~5 players with varied hp and positions.

    // 1. Regen: add 5 hp to every player (capped at 100). Use iter_mut().
    // 2. Names of all *living* players (hp > 0), collected into Vec<String>.
    // 3. Count how many players are within distance 10.0 of (0.0, 0.0).
    // 4. Find the player with the lowest hp (Option<&Player>).
    // 5. Remove all dead players (hp == 0) from the map.
    //    (Hint: collect dead ids first, then remove — you can't remove while iterating.)
}
```

**Done when:** each of the 5 operations works and you used iterators (not manual index loops) for
the queries. Step 5 teaches the "collect-then-mutate" pattern you'll use constantly in game loops.

---
<details><summary>Solution sketch</summary>

```rust
// 1.
for p in world.values_mut() { p.hp = (p.hp + 5).min(100); }
// 2.
let living: Vec<String> = world.values().filter(|p| p.hp > 0).map(|p| p.name.clone()).collect();
// 3.
let near = world.values().filter(|p| distance(p.pos, (0.0, 0.0)) < 10.0).count();
// 4.
let weakest = world.values().min_by_key(|p| p.hp);
// 5.
let dead: Vec<u32> = world.values().filter(|p| p.hp == 0).map(|p| p.id).collect();
for id in dead { world.remove(&id); }
```
</details>
