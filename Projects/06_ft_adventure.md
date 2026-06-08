```
                      ft_adventure
     A single-player text adventure engine (offline)
                    Version: 1.0
```

## Foreword

Time to make it a *game*. No sockets yet — one player, in your terminal, exploring the world from
project 03, using the commands parsed by project 02, picking up items from project 01, fighting,
and completing quests. This is where all the **game logic** lives. Keeping it offline means you can
get the rules right without network noise. In project 07 you'll wrap this exact engine in the
server from project 05.

## Objectives

- Combine your world, parser, and item model into a coherent game state machine.
- Implement movement, item take/drop, NPC dialogue, combat, and quests as **pure logic** over a
  `GameState` — no I/O mixed into the rules.
- Design the open-ended systems TAP leaves to you (combat formulas, quest progression) and
  document them.

## General rules

- Rust + Cargo. Norm clean.
- Reuse your earlier crates/modules (`protocol` from 02, `world` from 03, item model from 01).
- `Makefile`: `build`, `run`, `test`, `lint`, `clean`.

## Mandatory part

Build an engine exposing something like
`fn apply(state: &mut GameState, player: PlayerId, cmd: Command) -> Vec<Outcome>` — pure, testable,
no printing inside. A thin CLI loop reads lines → `parse` → `apply` → prints the `Outcome`s.

Implement the full mandatory command set acting on the world:

1. **LOOK** — current room description, exits, items on the ground, NPCs present.
2. **MOVE `<dir>`** — move if the exit exists; error otherwise.
3. **TAKE / DROP `<id|name>`** — unique-instance movement between room and inventory (from project
   01's rules). By id or display name; multi-word supported.
4. **INVENTORY** — list held items.
5. **TALK `<npc>`** — show the NPC's dialogue line(s).
6. **STATUS** — player HP and combat status.
7. **ATTACK `<npc>`** — your designed **combat system**:
   - players start at 100 HP; enemies have type-based HP;
   - `ATTACK` deals damage and may trigger a counter-attack;
   - at 0 HP a player respawns at a safe room with reduced HP;
   - document your damage formula, turn order, and any extra commands (`DEFEND`, `FLEE`).
8. **QUEST / QUESTS** — your designed **quest system**: at least 2 quests (e.g. fetch an item,
   defeat an NPC, deliver an item). Track progress, validate completion, grant a reward. Document
   the progression mechanics.
9. **Robustness:** invalid targets/directions/commands produce clean errors, never a panic.
10. **Tests** for the tricky logic: take/drop conservation, combat reducing HP and respawning,
    quest completing exactly once.

```
> talk guard
Village Guard: "Bring me the Healing Herbs and I'll reward you."
> quests
[1] Herb Errand — in progress (0/1 herbs delivered)
> take herbs
Took Healing Herbs.
> move north
> attack goblin
You hit Goblin for 18 (Goblin: 22 HP). Goblin hits you for 9 (You: 91 HP).
```

## Bonus part

- Group/party mechanics (shared quest progress) — preview of TAP's `GROUP`.
- NPC roles beyond the minimum: a merchant that trades, a quest-giver chain.
- A save/load of game state with `serde` (TAP doesn't require persistence, but it's good practice).

## Learning objectives (defend these)

- Why keep `apply()` pure (returns `Outcome`s) instead of printing inside the engine? (Hint:
  project 07 needs to turn those outcomes into *network events* for the right players — separation
  of logic from I/O is what makes that possible.)
- Walk through your combat formula. Is it deterministic or random? How do you test it?
- How does a quest know it's complete, and how do you guarantee the reward is granted once?

## How this feeds TAP

This is the entire **game-logic core** of the TAP server: items, combat, quests, NPCs, movement —
the parts the subject explicitly leaves to your design. After this, "the server" is just *this
engine, driven by many networked players instead of one keyboard.*

➡️ Next rung: `07_ft_shared_world.md`
