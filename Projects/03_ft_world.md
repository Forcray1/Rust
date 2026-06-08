```
                        ft_world
      Load, validate & walk a world from data files
                    Version: 1.0
```

## Foreword

A MUD's world is *data*, not code: rooms, exits, items, NPCs, quests — all in a YAML/JSON file an
author can edit without recompiling. TAP requires you to load static world data **and validate that
every exit and reference is correct**. A dangling exit (`north -> "tavren"` typo) should be caught
at load time with a clear error, not discovered when a player walks into the void.

## Objectives

- Deserialize structured game data with `serde` (`serde_yaml` and/or `serde_json`).
- Model a world as a graph: rooms keyed by id, exits as id references.
- Validate referential integrity before the game starts.
- Navigate the world from a CLI.

## General rules

- Rust + Cargo. Norm clean (`fmt` + `clippy`).
- `cargo add serde --features derive`, `cargo add serde_yaml serde_json`.
- `Makefile`: `build`, `run`, `test`, `lint`, `clean`.

## Mandatory part

1. **Data format.** Design your own YAML or JSON schema (the subject's tiny YAML is illustrative —
   your structure is your choice). It must express: locations (name, description, exits as
   `direction -> room_id`, items present, NPC spawns), items (id, name, description, obtainable),
   and NPCs (id, name, dialogue, stats like hp).
2. **Types + deserialization.** Define `World`, `Room`, `ItemDef`, `NpcDef` structs deriving
   `Deserialize`. Load the file into a `World` with `serde_yaml::from_str` / `serde_json::from_str`,
   returning a `Result` (a malformed file is an error, not a panic).
3. **Validation pass** — after loading, check and report **all** problems (don't stop at the first):
   - every exit target is a real room id;
   - every item/NPC referenced by a room exists in the item/NPC tables;
   - there are no unreachable rooms from the start (optional but recommended).
   Print a clear list of errors and exit non-zero if any.
4. **A CLI walker.** Start in the spawn room. Support `look` (name, description, exits, items,
   NPCs present) and `move <direction>`. Refuse invalid directions with a clean message.
5. **A real map.** Meet TAP's world size while you're here: **≥ 8 interconnected rooms forming at
   least one loop, plus an optional branch** (movement must allow a full circuit, not a line),
   **≥ 4 items (≥2 obtainable in-world)**, **≥ 3 NPC roles**, and stub **≥ 2 quests**. Building the
   real world now means TAP's "World Design" is already done.

```
$ cargo run -- worlds/world.yaml
Loaded world: 9 rooms, 5 items, 4 NPCs. Validation OK.
> look
Village Square — A bustling square with cobblestone paths.
Exits: north (tavern), east (shop), south (gate)
Items: Healing Herbs
NPCs: Village Guard
> move north
You go north.
> look
The Prancing Pony — A cozy tavern filled with warmth and laughter.
Exits: south (square)
```

## Bonus part

- A `validate`-only mode (`cargo run -- --check worlds/world.yaml`) that reports issues and exits —
  a linter for your world files.
- Support both YAML and JSON, chosen by file extension.
- Render an ASCII map of room connections.

## Learning objectives (defend these)

- Why store rooms in a `HashMap<String, Room>` keyed by id, with exits as `String` ids, rather than
  rooms holding `Box<Room>` pointers to each other? (Hint: ownership, cycles, `Rc`/`Weak` — course
  09. Id-indirection sidesteps the whole problem.)
- How does `serde`'s `Deserialize` derive turn a text file into typed structs with no manual
  parsing?
- What does your validation guarantee the rest of the game can now *assume*?

## How this feeds TAP

This delivers TAP's **world loading + validation** requirement and your entire **World Design**
section, standalone. Project 06 plugs the parser (02) into this world to make it playable; project
07 makes it multiplayer.

➡️ Next rung: `04_ft_echo_server.md`
