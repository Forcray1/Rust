```
                     ft_shared_world
   The multiplayer server: chatroom × adventure engine
                    Version: 1.0
```

## Foreword

The convergence. Project 05 gave you many clients sharing live state and events. Project 06 gave
you a pure game engine. Bolt them together: each TCP client is a player; each incoming line is
`parse`d into a `Command` and fed to `apply()`; each `Outcome` becomes a protocol reply to the actor
and `EVT`s pushed to the right other players. **This is the TAP server's mandatory part.**

## Objectives

- Drive the project-06 engine from many concurrent network connections.
- Route outcomes: replies to the actor, scoped events (room/global/group) to others.
- Add the production concerns TAP demands: structured logging and abuse monitoring.

## General rules

- Rust + Cargo, Tokio. Norm clean.
- Compose your existing crates: `protocol` (02), `world` (03), engine (06), networking (04/05).
- `Makefile`: `install`, `build`, `run-server`, `lint`, `clean`.

## Mandatory part

1. **Shared authoritative world.** One owner of the `GameState`. Recommended: a central **game
   task** that owns the state and receives `(PlayerId, Command)` events over a channel; connection
   tasks parse lines and forward them (course 10 architecture B). Alternative: `Arc<Mutex<World>>`.
   Document and justify your concurrency model — TAP's README requires this.
2. **Connection lifecycle.** `CONNECT <name>` registers a player (reject duplicate names), spawns
   them at the start room, and broadcasts presence. On disconnect: **remove player state first,
   then broadcast the leave event** (and drop any items per your rules).
3. **Command handling.** Every mandatory verb works over the network, producing protocol-compliant
   `OK ...` / `ERR <code> ...` replies (reuse project 02's encoder) and the `EVT` events from the
   subject:
   - `EVT ROOM PRESENCE ENTER/LEAVE <name>` on movement,
   - `EVT GLOBAL CHAT <name> <text>` and room/group-scoped chat,
   - combat results broadcast to relevant players, etc.
4. **Event scoping.** A move's presence events go to the old and new rooms; room chat only to that
   room; combat logs to involved players. Translate engine `Outcome`s → targeted network events.
5. **Structured logging** (a full TAP requirement): JSON lines with timestamps + levels
   (INFO/WARN/ERROR) for: connects/disconnects (with IP), every command received (player +
   params), every reply/error sent, world-state changes (item moves, combat, quests), and quest
   completion. Use the `tracing` crate (`tracing` + `tracing-subscriber` with JSON). Logging must
   not stall the server.
6. **Abuse monitoring.** Detect & log (WARN) command flooding and rapid reconnects. Optionally
   throttle.
7. **Robustness:** malformed lines → protocol error, never a crash; a client dying mid-broadcast
   doesn't disturb others.

```
$ cargo run --bin server -- worlds/world.yaml
{"ts":"2026-06-08T10:00:01Z","level":"INFO","event":"connect","addr":"127.0.0.1:51234"}
{"ts":"2026-06-08T10:00:02Z","level":"INFO","event":"command","player":"alice","cmd":"MOVE north"}
{"ts":"2026-06-08T10:00:02Z","level":"INFO","event":"presence","room":"loc.tavern","kind":"ENTER","player":"alice"}
```

## Bonus part

- Hot-reload the world file without dropping connections.
- Metrics endpoint / periodic stats log (players online, commands/sec).
- Server-authoritative anti-cheat checks (reject impossible moves) with WARN logs.

## Learning objectives (defend these)

- Trace one `MOVE north` from bytes on the socket to events on two other clients. Which task does
  what, and where does the borrow checker force the design?
- Why is "remove state, then broadcast leave" non-negotiable under concurrency?
- How does owning the world in a single game task remove the need for locks — and what's the
  trade-off vs `Arc<Mutex>`?

## How this feeds TAP

This **is** the TAP server, mandatory part, including its logging and robustness rules. What remains
for TAP is the two clients (projects 08 and 09) and the README sections — which you've been drafting
all along.

➡️ Next rung: `08_ft_cli_client.md`
