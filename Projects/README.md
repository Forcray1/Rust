# Projects — the 42-style ladder to TAP

The `Courses/` teach concepts; the `Exercises/` drill them. **Projects** are how 42 actually makes
you learn: a ladder of small, self-contained programs, each one runnable, each one a *piece* you'll
later assemble into the big project.

The big project here is **TAP — The Answer Protocol** (the official subject you were given): a
shared-world retro text adventure with a TCP server, a CLI client, and a GUI client. Our endgame
twist: the GUI client will eventually render the world as a **2D tile map in the style of Pokémon
Black & White**.

You do **not** build TAP during these projects. You build the *skills and the parts*. By the last
rung, assembling TAP is mostly gluing together things you've already written and understood.

> 42 rule that matters most here: **you must be able to explain every line you turn in.** Use these
> subjects as if a peer will grade you and ask "why did you do it this way?" Build it, break it,
> understand it.

---

## The ladder

| # | Project | Build | Feeds into TAP | Courses |
|---|---------|-------|----------------|---------|
| 00 | `ft_guess` | a number-guessing CLI game | the build/run loop, input parsing | 00–04 |
| 01 | `ft_inventory` | an RPG character + item manager (CLI) | item system, player stats, combat math | 03–06, 09 |
| 02 | `ft_protocol` | a line-based command parser/encoder | **the RFC 42TAP protocol core** | 03, 04, 13 |
| 03 | `ft_world` | load + validate + walk a world from YAML/JSON | world data, rooms, exits, validation | 06, 08, 13 |
| 04 | `ft_echo_server` | an async TCP echo server + client | the network spine | 11, 12 |
| 05 | `ft_chatroom` | a multi-client TCP chat with broadcast | CHAT, events, presence, disconnects | 10, 11, 12 |
| 06 | `ft_adventure` | a **single-player** text adventure engine | all game logic: move/take/drop/talk/combat/quests | 02–06, 14 |
| 07 | `ft_shared_world` | merge 05 + 06 → the multiplayer server | **≈ the TAP server, mandatory part** | 10–14 |
| 08 | `ft_cli_client` | a polished, responsive CLI client | **TAP CLI client** | 11, 12 |
| 09 | `ft_tiles` | a 2D tile-map GUI client (Pokémon B/W style) | **TAP GUI client + the 2D goal** | 12, 13 + macroquad |
| 10 | `TAP` | the official project, assembled | — | all |

Each rung lists which `Courses/` modules it leans on. If a project fights you, the gap is usually
in that module — go back, then return.

---

## The 42 way to use these

1. **Read the whole subject first.** Don't start coding until you know what "done" means.
2. **Do the mandatory part fully** before touching bonus. Mandatory = the skill; bonus = the flex.
3. **Pass the "norm".** Our norm is: `cargo fmt` clean **and** `cargo clippy` with zero warnings.
   42 fails you on norm errors; train the habit now.
4. **Make a build tool.** Every project gets a `Makefile` (or documented cargo commands) with
   targets: `install`, `build`, `run`, `lint`, `clean`. TAP requires exactly this.
5. **Write a tiny README** per project: what it is, how to run it, one design choice you made and
   why. TAP's README is huge — practice the muscle on small ones.
6. **Defend it.** Before moving on, explain the project out loud (to a peer, a rubber duck, or in
   your README) as if being evaluated. If you can't explain a part, you don't own it yet.

---

## Group work (TAP is a 2–3 person project)

TAP's suggested split: one person on **server + CLI client**, another on **GUI client + world
design**. The ladder maps cleanly onto that:

- **Server/CLI track**: 02 → 03 → 04 → 05 → 07 → 08
- **GUI/World track**: 01 → 03 → 06 → 09

Both tracks share 03 (`ft_world`) and meet at 07 (`ft_shared_world`). If you're solo right now, do
the whole ladder — you'll understand both halves, which makes you the strongest teammate.

---

## On the 2D Pokémon Black & White goal

TAP only requires *a* GUI (any toolkit). Your stretch goal — a top-down 2D tile world like Pokémon
B/W — shapes which toolkit to pick. Recommendation, justified in project 09:

- **`macroquad`** — dead-simple 2D rendering (sprites, tilemaps, input). Easiest path to "walk a
  character around a tile grid." Great first GUI.
- **`egui`** (via `egui-macroquad`) — immediate-mode UI panels for inventory, chat, buttons that
  TAP's GUI requires. Pairs with macroquad.
- **`bevy`** — a full ECS game engine (ties back to Course 14). More power, steeper curve. The
  natural home if you later grow this into a real game.

Start with macroquad + egui. Graduate to bevy if/when the game grows.

---

Start at `00_ft_guess.md`. Climb one rung at a time.
