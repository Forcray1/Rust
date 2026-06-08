# Exercises

One file per course module (same number). Each has:
- **Drills** — small, focused tasks to make a concept stick.
- **A mini-project** — a slightly larger task that builds something you'll reuse in the MMORPG.
- **Hints** and a **"Done when…"** checklist.
- **Solution sketches** at the bottom (try first, peek after).

## How to run them

The fastest setup: one scratch project you reuse.

```bash
cargo new rust_practice
cd rust_practice
```

Then for each exercise, edit `src/main.rs` and `cargo run`. For exercises with tests, put them in
`src/main.rs` under a `#[cfg(test)] mod tests { ... }` block and `cargo test`.

Alternatively, for zero setup while experimenting, use the online playground:
https://play.rust-lang.org

For the networking/async exercises (11+) you'll add dependencies:
```bash
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json bincode
cargo add tokio-util --features codec
cargo add anyhow
```

## Golden rules

1. Make it **compile** first, then make it **correct**, then make it **clean**.
2. When the borrow checker rejects you, read the *entire* error. It usually names the fix.
3. It's fine to `.clone()` to get unstuck early. Note it, move on, refactor later.
4. Don't peek at solutions until you've fought a problem for real. The fight is the learning.

## Progression

| # | Topic | Mini-project |
|---|-------|-------------|
| 02 | Ownership/borrowing | fix-the-borrow-checker gauntlet |
| 03 | Structs/enums/match | a `Packet` enum + dispatcher |
| 04 | Error handling | a safe command parser |
| 05 | Traits/generics | `Damageable` + a generic container |
| 06 | Collections/iterators | a tiny inventory & query engine |
| 07 | Lifetimes | a borrowing tokenizer |
| 08 | Modules/cargo | split a project into a lib + bin |
| 09 | Smart pointers | shared counter, then shared world |
| 10 | Concurrency | multi-threaded tick with channels |
| 11 | Async/Tokio | concurrent tasks + a tick interval |
| 12 | Networking | a framed TCP chat server |
| 13 | Serde | the full protocol round-trip |
| 14 | ECS | a hand-rolled mini-ECS with systems |
| 15 | Capstone | multiplayer movement+chat (Stages 0→4) |
