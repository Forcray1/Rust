```
                     ft_cli_client
       A responsive, real-time CLI client for TAP
                    Version: 1.0
```

## Foreword

A networked client has a deceptively hard requirement: it must **stay responsive while events
arrive asynchronously**. The player is mid-typing a command when `* bob entered` and a chat line and
a combat result all land — and none of it should freeze the prompt or get garbled. You already
proved the pattern in project 04; now make it pleasant and protocol-complete.

## Objectives

- Talk the full RFC 42TAP protocol to your project-07 server.
- Read user input and incoming server events **concurrently**, without one blocking the other.
- Optionally translate friendly commands into protocol lines.

## General rules

- Rust + Cargo, Tokio. Norm clean.
- Reuse the `protocol` crate (project 02) for encoding commands / decoding replies & events.
- `Makefile`: `build`, `run-client`, `lint`, `clean`.
- **Interoperability:** your CLI client must work against *another group's* server (and vice-versa).
  Stick to the protocol; document any deviation.

## Mandatory part

1. **Connect & authenticate:** connect to a host:port (CLI args), send `CONNECT <name>`, handle the
   `OK`/`ERR` reply.
2. **Concurrent loop:** simultaneously
   - read lines from stdin, turn them into protocol commands, send them; and
   - receive server lines (replies **and** `EVT` events) and display them in real time.
   Use two `tokio` tasks or a `select!`. The prompt must never lock up waiting on the network.
3. **Full command coverage:** be able to issue every mandatory verb (`LOOK`, `MOVE`, `CHAT`,
   `TAKE`, `DROP`, `INVENTORY`, `TALK`, `ATTACK`, `STATUS`, `QUEST`, `QUESTS`, `WHO`, `GROUP`,
   `QUIT`).
4. **Readable output:** pretty-print `LOOK` payloads (room, exits, items, NPCs), chat with scope
   tags, presence and combat events. Distinguish a reply to *your* command from a pushed *event*.
5. **Command Interface Choice (TAP):** pick one and document it —
   (a) raw mode: user types protocol syntax directly; or
   (b) friendly mode: user types `go north` / `take herbs` and the client translates to
   `MOVE north` / `TAKE herbs`. Friendly mode is more work but better UX.
6. **Graceful quit:** `QUIT` (or Ctrl-C) closes the connection cleanly.

```
$ cargo run --bin client -- 127.0.0.1:4000 alice
connected as alice
> look
== Village Square ==
A bustling square with cobblestone paths.
Exits: north, east, south   Items: Healing Herbs   NPCs: Village Guard
* bob entered the square
bob (room): anyone seen the goblin?
> chat global hi all
```

## Bonus part

- Command history & line editing (`rustyline` crate).
- Tab-completion of verbs / exits / item names.
- Colored output (chat scopes, errors) with `owo-colors` or ANSI codes.
- A status bar showing HP and current room.

## Learning objectives (defend these)

- Show precisely how your client avoids blocking: what runs when the user is idle vs typing?
- If you chose friendly mode, where does the translation happen and how do you keep it lossless for
  multi-word args?
- Why does reusing the shared `protocol` crate make client/server interop reliable?

## How this feeds TAP

This is TAP's **CLI client**, done. Because it speaks pure RFC 42TAP through the shared crate, it
will also drive *other groups'* servers — exactly the interoperability the subject demands.

➡️ Next rung: `09_ft_tiles.md` — go graphical, Pokémon-style.
