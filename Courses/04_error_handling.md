# 04 — Error Handling: Option & Result

A server must **never crash because a client sent garbage.** Rust has no exceptions; errors are
ordinary values you must handle. This feels verbose at first and becomes one of your favorite
features — every possible failure is visible in the types.

## `Option<T>`: a value, or nothing (the death of null)

```rust
enum Option<T> { Some(T), None }
```

Anywhere a value might be absent, the type says so. There is no `null` to forget to check.

```rust
fn find_player(world: &World, id: u32) -> Option<&Player> {
    // returns Some(player) or None
}

match find_player(&world, 42) {
    Some(p) => println!("found {}", p.name),
    None    => println!("no such player"),
}
```

Compare:
- **Python**: returns `None` or the value, but nothing forces you to check — `AttributeError` at
  runtime if you forget.
- **C**: returns `NULL` or a sentinel; forget to check → segfault.
- **Rust**: the type is `Option<&Player>`. You *cannot* use the player without unwrapping the
  Option first. The compiler enforces the check.

### Working with Option ergonomically

```rust
let hp: Option<u32> = Some(100);

hp.unwrap();                 // returns 100, but PANICS if None — avoid in real code
hp.unwrap_or(0);             // 100, or 0 if None
hp.unwrap_or_else(|| compute_default());
hp.map(|v| v * 2);           // Some(200) — transform the inner value if present
hp.is_some();                // true
hp.is_none();                // false

if let Some(v) = hp { /* use v */ }
```

## `Result<T, E>`: success or error (the death of exceptions)

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

Any operation that can fail returns a `Result`. The error type `E` describes what went wrong.

```rust
use std::num::ParseIntError;

fn parse_level(s: &str) -> Result<u8, ParseIntError> {
    s.parse::<u8>()          // .parse() returns a Result
}

match parse_level("12") {
    Ok(level) => println!("level {level}"),
    Err(e)    => println!("bad input: {e}"),
}
```

There's no hidden control flow like a thrown exception jumping up the stack. The error is right
there in the return type — you can see at a glance which functions can fail.

## The `?` operator — the ergonomic core

Manually matching every Result is painful. The `?` operator means: *"if this is `Err`, return it
from the current function; otherwise give me the `Ok` value."*

```rust
fn load_config(path: &str) -> Result<Config, std::io::Error> {
    let text = std::fs::read_to_string(path)?;   // ? : on error, return early with the error
    let cfg = parse(&text)?;                      // chains naturally
    Ok(cfg)
}
```

Without `?`, that's three nested matches. With it, it reads like Python's "happy path", but every
failure is still handled (propagated to the caller). `?` also works on `Option` (returns `None`
early).

> Mental model: `?` is "try this; on failure, bail out and hand the error upward." The caller then
> decides what to do. Errors bubble up *explicitly*, one `?` at a time.

## panic! — for unrecoverable bugs only

```rust
panic!("this should never happen: {x}");
```

A panic unwinds the thread and prints a message. Use it for *programmer errors* (broken
invariants), **not** for expected failures like bad network input. Things that panic:
`unwrap()` on `None`, array out-of-bounds, integer overflow in debug, explicit `panic!`.

**For an MMORPG server:** a panic in one connection's task should not take down the server. You'll
isolate connections (each in its own task/thread) and use `Result` for anything driven by client
input. Reserve panic for "the code is wrong," never "the client is wrong."

## Custom error types

Real projects define their own error enum:

```rust
#[derive(Debug)]
enum GameError {
    PlayerNotFound(u32),
    NotEnoughMana { needed: u32, have: u32 },
    InvalidMove,
}

fn cast_spell(p: &mut Player, cost: u32) -> Result<(), GameError> {
    if p.mana < cost {
        return Err(GameError::NotEnoughMana { needed: cost, have: p.mana });
    }
    p.mana -= cost;
    Ok(())   // () is the "unit" type — "succeeded, no meaningful value"
}
```

For `?` to convert between error types automatically, you implement the `From` trait (module 05),
or use a library:

- **`thiserror`** — easily derive nice error enums for libraries.
- **`anyhow`** — one catch-all error type for applications/prototypes (`anyhow::Result<T>`).

While learning/prototyping, `anyhow` removes a lot of friction:

```rust
// Cargo.toml: anyhow = "1"
use anyhow::Result;

fn run() -> Result<()> {
    let data = std::fs::read_to_string("world.json")?;   // any error type works with ?
    // ...
    Ok(())
}
```

## main() can return Result

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load_config("config.toml")?;
    Ok(())
}
```

## Summary mindset

| Situation | Tool |
|-----------|------|
| Value might be missing | `Option<T>` |
| Operation might fail with a reason | `Result<T, E>` |
| Propagate a failure to the caller | `?` |
| Provide a fallback | `unwrap_or`, `unwrap_or_else` |
| Truly impossible / a bug | `panic!` / `unwrap` / `expect` |

The golden rule for a networked game: **client input → always `Result`. Internal invariant
violation → `panic`.**

---

🏋️ Do `Exercises/04_errors.md`.

➡️ Next: `05_traits_and_generics.md`
