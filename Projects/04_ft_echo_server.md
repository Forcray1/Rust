```
                     ft_echo_server
        Your first async TCP server (+ client)
                    Version: 1.0
```

## Foreword

Now the network. Strip away the game entirely: a server that accepts TCP connections and echoes
back whatever lines it receives. This is the spine every networked program shares — accept, spawn,
read, write, handle disconnect. Get the spine solid and TAP's server is "the echo loop, but it
parses and acts instead of echoing."

## Objectives

- Run an async TCP server with **Tokio**.
- Handle each connection as an independent task.
- Read **line-framed, UTF-8** input (the TAP framing) — not raw byte chunks.
- Detect and handle disconnects cleanly.

## General rules

- Rust + Cargo. Norm clean.
- `cargo add tokio --features full`. (Line framing: either `BufReader::lines()` or
  `tokio-util` + `LinesCodec` — your choice, document it.)
- `Makefile`: `build`, `run-server`, `run-client`, `lint`, `clean`.

## Mandatory part

1. **Server** (`src/bin/server.rs`):
   - Bind `127.0.0.1:4000`, accept connections in a loop.
   - `tokio::spawn` a task per connection.
   - Read input **one line at a time** (`\n`-terminated, UTF-8). For each line, send it back
     prefixed, e.g. `echo: <line>\n`.
   - On EOF / read error, log a clean disconnect and end the task **without killing the server**.
   - Log connect/disconnect with the peer address.
2. **Client** (`src/bin/client.rs`):
   - Connect to the server, then concurrently: read lines from stdin and send them, **and** print
     anything the server sends. (Two tasks, or a `select!` — both must work at once. This is the
     "stay responsive while receiving" requirement in miniature.)
3. **Prove concurrency:** two clients connected simultaneously both get echoed, and one client
   quitting doesn't disturb the other or the server.

```
# terminal 1
$ cargo run --bin server
[INFO] listening on 127.0.0.1:4000
[INFO] 127.0.0.1:51234 connected
# terminal 2
$ cargo run --bin client
hello
echo: hello
```

(You can also test the server with `nc 127.0.0.1 4000`.)

## Bonus part

- Graceful shutdown on Ctrl-C (`tokio::signal::ctrl_c`) that says goodbye to clients.
- A connection counter logged on every connect/disconnect.
- Configurable bind address/port via a CLI argument.

## Learning objectives (defend these)

- **Why must framing be line-by-line?** Show that a single `read()` can return half a line or two
  lines, and explain how your line reader fixes it. (This is *the* classic TCP bug — course 12.)
- Why `tokio::spawn` per connection instead of one big loop? What would block what?
- Why does the client need to read the socket and stdin *concurrently* rather than in sequence?

## How this feeds TAP

This is the literal accept/spawn/read-line/respond loop of the TAP server, and the
read-while-typing structure of both TAP clients. Project 05 turns "echo" into "broadcast"; project
07 turns the line into a parsed `Command` (from project 02) acting on the world (project 03).

➡️ Next rung: `05_ft_chatroom.md`
