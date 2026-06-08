# 12 — Networking (TCP / UDP)

The actual wire between client and server. We'll use Tokio's async networking. The concepts (TCP vs
UDP, framing) carry over from C sockets — but Tokio gives you a far nicer, safe API.

## TCP vs UDP — which for an MMORPG?

| | TCP | UDP |
|--|-----|-----|
| Delivery | reliable, ordered | best-effort, may drop/reorder |
| Connection | yes (stream) | no (datagrams) |
| Overhead | higher (acks, retransmit) | lower, faster |
| Use for | login, chat, inventory, anything that must arrive | fast position updates where stale data is useless |

Real MMORPGs often use **TCP for everything to start** (simpler, correct), then add **UDP** for
high-frequency state (movement) once it matters. **Start with TCP.** Get it working, then optimize.

## A minimal async TCP echo server

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("listening on :4000");

    loop {
        let (mut socket, addr) = listener.accept().await?;   // wait for a client
        println!("connection from {addr}");

        tokio::spawn(async move {                            // one task per client
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,                          // 0 bytes = client closed
                    Ok(n) => {
                        // echo back what we got
                        if socket.write_all(&buf[..n]).await.is_err() { break; }
                    }
                    Err(_) => break,
                }
            }
            println!("{addr} disconnected");
        });
    }
}
```

This is the skeleton of every TCP server: bind → accept loop → spawn a task per connection. Test it
with `nc 127.0.0.1 4000` and type.

## A minimal client

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000").await?;
    stream.write_all(b"hello server").await?;

    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("got: {}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
```

## The framing problem (critical, and everyone hits it)

TCP is a **byte stream**, not a message stream. One `read()` may return *half* a message, *two*
messages, or a message split across reads. You must define where one message ends and the next
begins — this is **framing**.

Two common schemes:

1. **Length-prefixed** (most common for binary): write a 4-byte length, then the payload. To read:
   read exactly 4 bytes → that's N → read exactly N more bytes → that's one full message.
2. **Delimiter-based** (e.g. newline-terminated): good for text/JSON-lines protocols.

Do **not** assume one `read()` == one message. That bug works on localhost and breaks in
production.

### Don't hand-roll framing — use `tokio_util::codec`

Tokio's `codec` + the `Framed` type turn a byte stream into a stream of *messages* for you:

```toml
tokio-util = { version = "0.7", features = ["codec"] }
```

```rust
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use futures::StreamExt;   // for .next()

let mut framed = Framed::new(socket, LengthDelimitedCodec::new());
while let Some(frame) = framed.next().await {
    let bytes = frame?;                 // one complete, length-delimited message
    let msg: ClientMessage = decode(&bytes)?;   // deserialize (module 13)
    // handle msg
}
```

`LengthDelimitedCodec` handles the read-exactly-N dance. You pair it with serde/bincode (module 13)
to go from bytes ↔ your `ClientMessage`/`ServerMessage` enums. For newline-delimited text there's
`LinesCodec`.

## Putting the server architecture together

Combining modules 10–13, a TCP MMORPG server looks like:

```
                        ┌─────────────────────────────┐
   client ──TCP──┐      │  game task (owns World)     │
   client ──TCP──┼─tx─▶ │  every 50ms tick:           │
   client ──TCP──┘      │   - drain incoming actions  │
        ▲               │   - update world            │
        │               │   - broadcast state ────────┼─┐
        └───────────────┴─────────────────────────────┘ │
              per-client task: select! { read socket,    │
                                          recv broadcast }◀┘
```

- Each connection: a Tokio task. It deserializes incoming frames into `ClientMessage` and forwards
  them to the game task via an `mpsc` channel.
- The game task: owns the `World`, runs the tick loop, and pushes `ServerMessage` updates out via a
  `broadcast` channel (or per-client `mpsc`).
- Each connection task `select!`s between "client sent a packet" and "server has an update for me."

This is exactly what you'll build in the capstone (module 15).

## Practical tips

- Start on `127.0.0.1` (localhost). Worry about real network/NAT later.
- Use `tracing`/`println!` generously to see connection lifecycle while learning.
- Handle disconnect everywhere (`Ok(0)`, errors) — clients vanish constantly; never `unwrap()` on
  socket ops in a way that kills the server.
- Test with two terminals (server + client binary), or `nc` for raw bytes.

---

🏋️ Do `Exercises/12_networking.md`.

➡️ Next: `13_serialization_serde.md`
