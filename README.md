# Rust → MMORPG Learning Path

A curriculum built for someone who **already knows Python and C**, aimed at one concrete goal:
**be able to architect and build an MMORPG server + client in Rust.**

We don't rebuild what you already know. We move fast through the familiar (loops, functions,
arithmetic) and spend our time on what's genuinely *new* in Rust — the stuff that trips up every
Python/C programmer and the stuff an MMORPG actually depends on.

---

## The three folders

- **`Courses/`** — concepts, in order, focused on what's new vs Python/C.
- **`Exercises/`** — short drills + mini-projects to make each concept stick (same numbering).
- **`Projects/`** — a **42-style ladder of small, runnable projects**, each a piece you'll later
  assemble into the real target project: **TAP — The Answer Protocol** (a shared-world text
  adventure: TCP server + CLI client + GUI client), with the GUI eventually rendered as a **2D tile
  world in the style of Pokémon Black & White**. Start at `Projects/README.md`.

## How to use this

1. Read a course module in `Courses/` (they're numbered — do them in order).
2. Do the matching exercises in `Exercises/` (same number).
3. Every few modules, build the corresponding rung in `Projects/` — that's where real understanding
   happens (42 learns by building, not reading).
4. Run your code with `cargo` (see `Courses/00_setup.md`).
5. Don't move on until it compiles **and** you can *explain why* — in 42, you must defend every line.

> The Rust compiler is your tutor. When it rejects your code, it's almost always teaching you
> something true about memory or ownership. Read the error messages fully — they're the best in
> the industry.

---

## The modules

| # | Module | Why it matters for an MMORPG |
|---|--------|------------------------------|
| 00 | Setup & toolchain | cargo, the build/run loop |
| 01 | Basics vs Python & C | the syntax differences, fast |
| 02 | **Ownership & borrowing** ⭐ | the #1 new idea; underlies *everything* |
| 03 | Structs, enums & pattern matching | model players, items, packets, game states |
| 04 | Error handling (Option/Result) | servers must never crash on bad input |
| 05 | Traits & generics | shared behavior for entities, abilities, systems |
| 06 | Collections & iterators | world state, inventories, spatial queries |
| 07 | Lifetimes | borrow data across function boundaries safely |
| 08 | Modules, crates & Cargo | structure a large codebase, pull in libraries |
| 09 | Smart pointers (Box/Rc/Arc/RefCell) | shared mutable game state |
| 10 | **Concurrency: threads & channels** ⭐ | handle many players at once |
| 11 | **Async & Tokio** ⭐ | thousands of concurrent network connections |
| 12 | Networking (TCP/UDP) | the actual client↔server link |
| 13 | Serialization (serde) | turn game data into packets and back |
| 14 | Game architecture & ECS | how real game engines are organized |
| 15 | Capstone: toward the MMORPG | combine everything into a tiny game server |

⭐ = the hard, high-value modules. Don't rush these.

---

## What an MMORPG needs, and where you learn it

- **Players connecting/disconnecting constantly** → async networking (11, 12)
- **Shared world state mutated by many connections** → ownership + Arc/Mutex (02, 09, 10)
- **Messages over the wire** → enums + serde (03, 13)
- **A game loop ("tick") updating all entities** → ECS + iterators (06, 14)
- **Never crashing on malformed input** → Result/Option (04)
- **Performance** → Rust gives this for free vs Python, *if* you avoid needless cloning (02, 06)

---

## Suggested pace

| Pace | Plan |
|------|------|
| Intense | 1 module/day → ~2 weeks + capstone |
| Steady | 2–3 modules/week → ~6 weeks |
| Casual | 1 module/week → ~4 months |

Don't skip exercises. Reading Rust feels easy; *writing* it is where ownership becomes real.

---

## Reference links (bookmark these)

- The Book (official, excellent): https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings (interactive exercises): https://github.com/rust-lang/rustlings
- Standard library docs: https://doc.rust-lang.org/std/
- For games specifically — `bevy` engine: https://bevyengine.org/
