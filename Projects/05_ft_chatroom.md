```
                      ft_chatroom
       A multi-client TCP chat with broadcast & presence
                    Version: 1.0
```

## Foreword

Echo talks to one client at a time. A *world* needs every client to see what others do: chat,
arrivals, departures. That's **broadcast** and **presence** — and handling the ugly case where a
client vanishes mid-broadcast without taking the server (or other clients) down. TAP's `CHAT` and
its `EVT ... PRESENCE ENTER/LEAVE` events are exactly this project.

## Objectives

- Fan a message out to all (or a subset of) connected clients.
- Track shared connection state safely across tasks.
- Broadcast **presence** events on join/leave.
- Remove a player's state **before** announcing their departure (a TAP global rule).

## General rules

- Rust + Cargo, Tokio. Norm clean.
- Shared state: a `tokio::sync::broadcast` channel and/or `Arc<Mutex<…>>` (course 09/10/11).
  Document which architecture you chose and why.
- `Makefile`: `build`, `run-server`, `run-client`, `lint`, `clean`.

## Mandatory part

1. **Naming:** on connect, a client sends a name (e.g. first line, or `CONNECT <name>`). Reject
   duplicate names with an error line.
2. **Broadcast chat:** any line a client sends (that isn't a command) is broadcast to **all other**
   clients as `<name>: <text>`. The sender sees an `OK`-style ack, not their own echo (your call —
   document it).
3. **Presence events:** when a client joins, broadcast `* <name> joined`; when one leaves (quit or
   disconnect), broadcast `* <name> left`.
4. **State removal ordering:** on disconnect you must remove the player from the shared roster
   **first**, *then* broadcast the leave event. (No phantom players in `who`.)
5. **`/who` command:** lists currently connected names and the total count.
6. **Robustness:** a client crashing/Ctrl-C'ing mid-session must not panic the server or block
   broadcasts to others. Test with 3 clients; kill one rudely.

```
# client "bob" sees:
* alice joined
alice: hi everyone
* alice left
```

## Bonus part

- **Scoped chat** like TAP: `GLOBAL` (everyone) vs `ROOM` (same room) — fake "rooms" by letting
  clients `/join <room>` and only broadcasting room chat to that room. (This previews TAP's
  per-room broadcasting and sets up project 07.)
- A `/group` mechanic: form a group, `GROUP` chat goes only to members.
- Rate-limit floods (preview of TAP's "abuse pattern" logging) — drop/throttle a client sending too
  fast, and log it.

## Learning objectives (defend these)

- Compare the two architectures from course 10: `Arc<Mutex<roster>>` + per-client senders vs a
  central task owning state with a `broadcast` channel. Which did you pick and what does it cost?
- How do you avoid one slow/dead client stalling the broadcast to everyone else?
- Why does the *order* (remove state, then announce leave) matter? Construct the race that the wrong
  order creates.

## How this feeds TAP

You now have TAP's **CHAT**, **presence events**, **WHO**, graceful disconnect handling, and the
seed of **scoped/room/group chat** — the whole "many people share a live world" machinery, minus
the game. Project 07 drops the game logic in.

➡️ Next rung: `06_ft_adventure.md`
