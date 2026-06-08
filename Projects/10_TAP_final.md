```
                         TAP
        The Answer Protocol — assemble & ship
                    Version: 1.0 (maps to subject v1.4)
```

## Foreword

You didn't build TAP in one heroic sprint — you built its parts, understood each, and can defend
each. This rung is **integration and polish**: bring the pieces into one repository, fill every
mandatory requirement and README section from the official subject, and prepare for peer
evaluation. Nothing here is new code you haven't practiced; it's assembly + proof.

> The official subject (TAP v1.4) is the source of truth. This file is your *assembly checklist*
> mapping your ladder work onto its requirements. Read the real subject's RFC 42TAP for exact
> ABNF, error codes, and message formats — your project 02 must match them precisely.

## Where each deliverable comes from

| TAP deliverable | Built in |
|-----------------|----------|
| Server (all commands + events) | `ft_shared_world` (07), using `ft_protocol` (02), `ft_world` (03), `ft_adventure` (06), `ft_chatroom` (05), `ft_echo_server` (04) |
| Item system (unique instances) | `ft_inventory` (01) → 06 → 07 |
| Combat system | `ft_adventure` (06) → 07 |
| Quest system | `ft_adventure` (06) → 07 |
| World data (YAML/JSON) + validation | `ft_world` (03) |
| Structured logging + abuse monitoring | `ft_shared_world` (07) |
| CLI client | `ft_cli_client` (08) |
| GUI client (2D Pokémon-style) | `ft_tiles` (09) |
| Build tool (Cargo + Makefile targets) | every project; consolidate here |

## Repository layout (suggested)

```
tap/
├── Cargo.toml                # workspace
├── Makefile                  # install / build / run-server / run-client / run-client-gui / lint / clean
├── README.md                 # all required sections (below)
├── crates/
│   ├── protocol/             # project 02 — shared by everything (RFC 42TAP)
│   ├── world/                # project 03 — data model + loader + validator
│   └── engine/               # project 06 — pure game logic (items, combat, quests)
├── server/                   # project 07 — bin
├── cli-client/               # project 08 — bin
├── gui-client/               # project 09 — bin (macroquad + egui)
└── worlds/
    └── world.yaml            # ≥8 rooms (a loop + branch), ≥4 items, ≥3 NPC roles, ≥2 quests
```

A Cargo **workspace** lets the three binaries share `protocol`/`world`/`engine` with no
duplication — the clean realization of course 08.

## Final mandatory checklist (from the subject)

**Server**
- [ ] Every command: CONNECT, LOOK, MOVE, CHAT, TAKE, DROP, INVENTORY, TALK, ATTACK, STATUS,
      QUEST, QUESTS, WHO, GROUP, QUIT.
- [ ] Every event format and error code per RFC 42TAP; malformed input → compliant error, never a
      crash.
- [ ] Loads + validates world data; reports bad exits/references.
- [ ] Item system: unique instances, take removes, drop returns, by id or name, multi-word names.
- [ ] Combat: 100 HP start, enemy HP varies, ATTACK + counter, STATUS, respawn at 0 HP, results
      broadcast.
- [ ] Quests: ≥2, progression + completion validation + rewards.
- [ ] Removes player state **before** broadcasting leave; broadcasts survive a mid-send disconnect.
- [ ] Structured (JSON) logging with levels + timestamps for connects, commands, replies, state
      changes, quest events; abuse-pattern (flood/rapid-connect) monitoring.

**CLI client**
- [ ] Real-time display; stays responsive while receiving events; full command coverage; documented
      raw-vs-friendly interface choice.

**GUI client**
- [ ] Room/items/NPCs/exits with real-time updates; inventory + TAKE/DROP buttons; room view
      auto-updates after take/drop; separated chat (Global/Room/Group) and log views; action
      buttons; player counters (room + server); NPC dialogue via TALK. (Not curses — a true GUI.)

**Interop**
- [ ] CLI and GUI clients work against other groups' servers (and your server with theirs).

**Build & norm**
- [ ] Makefile targets: install deps, run-server, run-client, run-client-gui, lint, clean.
- [ ] `cargo fmt` clean, `cargo clippy` zero warnings.

## README.md required sections (from the subject)

First line, italicized:
`*This project has been created as part of the 42 curriculum by <login1>[, <login2>[, <login3>]].*`

Then: **Description**, **Instructions**, **Resources** (classic references + how AI was used and
for what), **Architecture** (dispatcher/router vs inline; concurrency model), **Protocol
Implementation** (any RFC deviations + justification), **Combat System**, **Quest System**, **World
Design**, **Server Logging**, **Group Contributions**, **Building and Running**, **Testing**. Written
in English.

> You've been drafting most of these in each project's mini-README and "defend these" sections.
> Now consolidate.

## Group split (2–3 learners)

- **Server + CLI track:** owns `protocol`, `engine` integration, `server`, `cli-client` (ladder
  02→03→04→05→07→08).
- **GUI + World track:** owns `world` data/design, `engine` logic, `gui-client` (ladder
  01→03→06→09).
- Both commit regularly (the subject checks per-member commits) and document who did what.

## Evaluation prep (the 42 part everyone underestimates)

- Expect a **live modification request**: "add a `WHISPER` command", "make combat use initiative",
  "change a payload field". Because your logic is modular (parser / engine / transport separated),
  small changes stay small. Rehearse one.
- Expect **"explain this"**: pick any file and be able to walk a peer through it. If a part was
  AI-assisted, you must still own it completely — that's both the 42 rule and the honest one.

## Beyond TAP — toward the full Pokémon B/W game

Once TAP passes, the natural growth path (and why we chose macroquad/bevy and an ECS-friendly
engine):

1. Migrate `engine` to a real **ECS** (course 14; `hecs` or `bevy`) for many entities + systems.
2. Richer tile worlds, NPCs with movement/AI, battles as a dedicated scene.
3. Binary protocol (`bincode` + length framing) and interest management for scale (course 12/13).
4. Persistence (serde save files / a database).

That's the same ladder logic, one rung higher. You now have every skill the climb requires.

🎉 Ship TAP. Then go make the addictive part (just... not *campus-blackout* addictive).
