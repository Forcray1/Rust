# Exercises 04 — Error Handling (Option & Result)

Servers live or die on this. Practice making bad input a *value*, not a crash.

## Drills

**D1.** Write `fn safe_div(a: i32, b: i32) -> Option<i32>` returning `None` on divide-by-zero.
Call it and print results with `unwrap_or(-1)`.

**D2.** Write `fn parse_coord(s: &str) -> Result<(i32, i32), std::num::ParseIntError>` that parses
`"3,5"` into `(3, 5)`. Use `?` for each parse. (Hint: `s.split_once(',')` then `.parse()`.)
Decide what to do if there's no comma — for now you may `.unwrap()` the split, then improve in the
mini-project.

**D3.** Given `let v = vec![10, 20, 30];`, use `.get(index)` (returns `Option`) instead of `v[i]`
to safely read index 5 without panicking.

**D4.** Chain with `?`: write `fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>>`
that reads a file to a string and parses it as an `i32`. Two `?` lines.

## Mini-project: a safe command parser

Parse player console commands of the form `"<verb> <args...>"`. The server must never panic on bad
input — every malformed command becomes a clean `Err`.

```rust
#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Quit,
}

#[derive(Debug)]
enum ParseError {
    Empty,
    UnknownVerb(String),
    BadArgs(String),
}

// TODO: parse a line like:
//   "move 3 5"  -> Ok(Command::Move { x: 3, y: 5 })
//   "say hello world" -> Ok(Command::Say("hello world".into()))
//   "quit" -> Ok(Command::Quit)
//   "" -> Err(ParseError::Empty)
//   "fly" -> Err(ParseError::UnknownVerb("fly".into()))
//   "move 3 abc" -> Err(ParseError::BadArgs(...))
fn parse(line: &str) -> Result<Command, ParseError> {
    todo!()
}

fn main() {
    for line in ["move 3 5", "say hello world", "quit", "", "fly", "move 3 abc"] {
        println!("{:?} -> {:?}", line, parse(line));
    }
}
```

**Done when:** all six inputs produce the expected `Ok`/`Err`, and `main` runs to completion with
no panic. Internalize: *every* path returns a value.

---
<details><summary>Solution sketch</summary>

```rust
fn parse(line: &str) -> Result<Command, ParseError> {
    let mut parts = line.split_whitespace();
    let verb = parts.next().ok_or(ParseError::Empty)?;
    match verb {
        "quit" => Ok(Command::Quit),
        "say" => {
            let rest: Vec<&str> = parts.collect();
            Ok(Command::Say(rest.join(" ")))
        }
        "move" => {
            let x = parts.next().and_then(|s| s.parse().ok())
                .ok_or_else(|| ParseError::BadArgs("x".into()))?;
            let y = parts.next().and_then(|s| s.parse().ok())
                .ok_or_else(|| ParseError::BadArgs("y".into()))?;
            Ok(Command::Move { x, y })
        }
        other => Err(ParseError::UnknownVerb(other.into())),
    }
}
```
`.ok_or(...)` turns an `Option` into a `Result`; `.ok()` turns a `Result` into an `Option`.
</details>
