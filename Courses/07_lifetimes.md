# 07 — Lifetimes

Lifetimes are the part of Rust that looks most alien (`<'a>` everywhere) but the *idea* is simple:
**the compiler tracks how long each reference is valid, to guarantee no reference outlives the data
it points to.** You met the consequence in module 02 ("no dangling references"). Lifetimes are the
notation for it.

Good news: you'll *read* lifetimes far more than you *write* them. The compiler infers most.

## The mental model

Every reference has a lifetime — the span during which it's valid. Lifetimes don't change runtime
behavior; they're purely a compile-time proof that pointers stay valid. There is no equivalent in
Python (GC handles it) or C (you're on your own, and dangling pointers are a top bug source).

```rust
let r;                     // r declared
{
    let x = 5;
    r = &x;                // r borrows x
}                          // x dropped here
println!("{r}");           // ❌ ERROR: x doesn't live long enough
```

The compiler sees `r` would point to freed memory and refuses. In C this compiles and is a
use-after-free.

## When you must write lifetime annotations

Only when the compiler can't figure out how output references relate to input references — almost
always in **function signatures that return references**, and in **structs that hold references**.

```rust
// "the returned reference lives as long as BOTH inputs" — we name that lifetime 'a
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`'a` is just a name (read "tick-a"). It doesn't pick a duration; it expresses a *relationship*:
"the output borrows from the inputs, so it can't outlive them." Without it, the compiler wouldn't
know whether the returned reference came from `a` or `b`, so it couldn't verify safety.

### Structs that hold references

```rust
struct Parser<'a> {
    input: &'a str,        // this struct borrows a string it doesn't own
    pos: usize,
}
```

The `<'a>` says: "a `Parser` cannot outlive the `&str` it borrows." Useful, but for an MMORPG you'll
often prefer structs that *own* their data (`String`, `Vec`) precisely to avoid threading lifetimes
through everything — owned data is simpler to move between threads and store in long-lived state.

## Lifetime elision — why you rarely write them

The compiler applies elision rules so the common cases need no annotation:

```rust
fn first_word(s: &str) -> &str { /* ... */ }   // output borrows from the single input — inferred
fn name(&self) -> &str { &self.name }          // output borrows from &self — inferred
```

You only annotate when there are multiple input references and the compiler can't tell which one
the output borrows from.

## `'static` — lives for the whole program

```rust
let s: &'static str = "hello";   // string literals are baked into the binary → 'static
```

`'static` means "valid for the entire program." String literals are `'static`. You'll also see it
as a bound (`T: 'static`) meaning "this type contains no short-lived borrows" — which matters when
sending data to threads/async tasks (modules 10–11), because those may outlive the current scope.

> When you spawn a thread or async task, the data it captures must be `'static` (own its data or be
> a literal/leaked). That's why game-server state is usually *owned* and shared via `Arc` (module
> 09), not passed as short-lived references.

## Practical guidance for your MMORPG

1. **Prefer owned data in long-lived structures.** Your `World`, `Player`, packet types should own
   their fields (`String`, `Vec`, ids) rather than borrow. This sidesteps most lifetime pain.
2. **Use references for short-lived borrows** within a function or a single tick — passing `&World`
   into a system, returning `&Player` from a lookup.
3. **When the compiler demands a lifetime,** read its suggestion. It usually wants you to either
   add `<'a>` linking input and output, or to return owned data (`String` instead of `&str`,
   `.clone()` / `.to_owned()`).
4. **Don't fight to avoid a clone if it tangles your lifetimes.** Correct-and-cloning beats
   clever-and-stuck. Optimize later.

## A worked example

```rust
struct World {
    players: std::collections::HashMap<u32, Player>,
}

impl World {
    // returns a borrow into self — lifetime elided (output borrows from &self)
    fn get(&self, id: u32) -> Option<&Player> {
        self.players.get(&id)
    }

    // returns owned data — no lifetime needed, caller fully owns the result
    fn name_of(&self, id: u32) -> Option<String> {
        self.players.get(&id).map(|p| p.name.clone())
    }
}
```

The first method hands out a borrow tied to `self`; the second hands out an independent `String`.
For data you'll send across threads or store, prefer the owned style.

## TL;DR

- Lifetimes = compile-time proof that references never dangle. No runtime cost.
- You mostly read them; write `<'a>` only when returning/storing references with ambiguous origin.
- For game state, **own your data** and you'll barely touch explicit lifetimes.

---

🏋️ Do `Exercises/07_lifetimes.md`.

➡️ Next: `08_modules_crates_cargo.md`
