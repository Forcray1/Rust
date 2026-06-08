# Exercises 12 — Networking (TCP)

Real sockets. Test with two terminals.

Setup: `cargo add tokio --features full` and `cargo add tokio-util --features codec` and
`cargo add futures`.

## Drill: echo server

Type out the echo server from the course (`Courses/12_networking.md`). Run it, then in another
terminal:
```bash
nc 127.0.0.1 4000
```
Type lines and watch them echo. Open a *second* `nc` — confirm both clients work simultaneously
(each is its own task). Ctrl-C a client and confirm the server logs the disconnect and stays up.

**Checkpoint questions:**
- What does `Ok(0)` from `read` mean? (client closed)
- Why is the per-connection work inside `tokio::spawn`? (so one slow/blocked client doesn't stall
  others)

## Mini-project: a framed line-based chat server

Build a chat server where every connected client sees every other client's messages. This is
Stage 3 of the capstone in miniature.

**Design:**
- Use a `tokio::sync::broadcast` channel to fan messages out to all clients.
- Each connection task:
  1. clones a `broadcast::Sender` and `subscribe()`s a `broadcast::Receiver`.
  2. wraps the socket in `Framed<_, LinesCodec>` so you read/write whole lines.
  3. loops on `tokio::select!`:
     - a line read from *this* client → broadcast it (prefixed with the client's addr).
     - a message received from the broadcast → write it to *this* client's socket.

**Skeleton to complete:**
```rust
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    let (tx, _rx) = broadcast::channel::<String>(100);
    println!("chat server on :4000");

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let mut framed = Framed::new(socket, LinesCodec::new());
            let _ = tx.send(format!("* {addr} joined"));
            loop {
                tokio::select! {
                    line = framed.next() => {
                        match line {
                            Some(Ok(msg)) => { let _ = tx.send(format!("[{addr}] {msg}")); }
                            _ => break,   // disconnected or error
                        }
                    }
                    Ok(msg) = rx.recv() => {
                        if framed.send(msg).await.is_err() { break; }
                    }
                }
            }
            let _ = tx.send(format!("* {addr} left"));
        });
    }
}
```

**Test:** run the server, connect two `nc 127.0.0.1 4000` clients, type in one, see it appear in
the other (and prefixed with the sender's address).

**Done when:** two+ clients chat through the server, join/leave is announced, and killing one
client doesn't affect the others or the server.

---
<details><summary>Notes & gotchas</summary>

- `cargo add anyhow` for the `anyhow::Result` return type on `main`.
- `framed.send(...)` needs `use futures::SinkExt;`; `framed.next()` needs `use futures::StreamExt;`.
- A slow client can lag the broadcast channel and get a `RecvError::Lagged` — for now you can treat
  any `Err` from `rx.recv()` as "skip/continue" or break. Real servers handle backpressure here.
- This *is* the chat layer of your MMORPG. Next module swaps the `String` payload for typed,
  serialized `ClientMessage`/`ServerMessage`.
</details>
