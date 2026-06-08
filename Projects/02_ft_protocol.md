```
                      ft_protocol
     A line-based command parser & encoder (RFC core)
                    Version: 1.0
```

## Foreword

TAP is defined by **RFC 42TAP**: a line-based, UTF-8, `\n`-terminated protocol. The single most
important — and most failed — part of a protocol project is parsing input *correctly and safely*.
A server that panics on a malformed line is a server an evaluator can crash in five seconds. Here
you build the protocol's brain with **zero networking**: just `String` in, typed value out, and a
reply `String` back.

## Objectives

- Parse untrusted text into a typed `enum Command`, returning protocol-compliant errors for
  garbage — never panicking.
- Encode typed responses back into protocol lines.
- Build the exact module both the server and clients will share.

## General rules

- Rust + Cargo. Norm: `cargo fmt` + `cargo clippy` clean.
- **Never panic on input.** Every parse path returns a `Result`.
- `Makefile`: `build`, `run`, `test`, `lint`, `clean`. (Yes, `test` — this project lives on tests.)

## Background: the protocol shape (from your TAP subject)

Client sends commands; server replies `OK ...` or `ERR <code> ...`; server also pushes `EVT ...`
events. Examples from the subject:

```
C: CONNECT alice
S: OK connected
C: MOVE north
S: OK room=loc.bakery
C: CHAT GLOBAL Hello everyone
S: OK
S: EVT GLOBAL CHAT alice Hello everyone
C: TAKE Herbs
S: OK taken=item.herbs
C: FOObar
S: ERR 400 unknown command
```

## Mandatory part

1. **Define `Command`** — an enum covering the mandatory verbs from the TAP subject:
   `CONNECT`, `LOOK`, `MOVE`, `CHAT`, `TAKE`, `DROP`, `INVENTORY`, `TALK`, `ATTACK`, `STATUS`,
   `QUEST`, `QUESTS`, `WHO`, `GROUP`, `QUIT`. Give each variant the arguments it needs (e.g.
   `Move { direction: String }`, `Chat { scope: ChatScope, text: String }`, `Take { target: String }`).
2. **Write `parse(line: &str) -> Result<Command, ProtoError>`:**
   - Split into verb + arguments (verb is case-insensitive per common MUD convention — document
     your choice).
   - `CHAT GLOBAL Hello everyone` must keep `Hello everyone` as one text field (don't lose spaces).
   - `TAKE Healing Herbs` must keep the multi-word target.
   - Unknown verb → `ERR 400 unknown command`. Missing/invalid args → an appropriate error code.
3. **Define `ProtoError`** with at least: unknown command, missing argument, bad argument. Each
   maps to a numeric code (e.g. 400, 422…). Document your code table.
4. **Write `encode_reply` / `encode_event`** that turn typed responses (`Reply::Ok`,
   `Reply::Err { code, msg }`, events like `EVT ROOM PRESENCE ENTER alice`) back into protocol
   lines ending in `\n`.
5. **Tests, tests, tests.** Provide a `#[cfg(test)]` module proving:
   - every verb parses;
   - multi-word names and chat text survive;
   - malformed lines produce the right error code and **never panic**;
   - round-trip: `encode(parse(line))` is stable for well-formed lines.

```
$ cargo test
running 14 tests ... ok
```

Also provide a small `main` that reads stdin lines and prints the parsed `Command` (or the error),
so you can poke at it by hand.

## Bonus part

- A `Reply`/`Event` enum hierarchy rich enough to serialize the JSON-ish payloads the subject shows
  (`OK { "room": {...}, "players":[...] }`) — pull in `serde_json` for the structured bits.
- A fuzz-ish test: feed thousands of random byte strings and assert the parser never panics.
- Strict ABNF compliance notes mapping each rule to your parser.

## Learning objectives (defend these)

- Show the exact line of code that prevents a panic on empty / malformed input.
- How does `split_whitespace` vs `splitn` vs `split_once` change whether multi-word args survive?
- Why is putting this in its own module (later: its own lib crate) the right call for a project with
  three binaries (server, CLI, GUI)?

## How this feeds TAP

This *is* the heart of RFC 42TAP. The server will call your `parse` on every incoming line; both
clients will use your `encode_*` to talk to the server. Get this bulletproof now and the rest of
TAP is plumbing.

➡️ Next rung: `03_ft_world.md`
