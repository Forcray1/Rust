# Exercises 07 — Lifetimes

Mostly about reading and satisfying the compiler. Keep it light — then prefer owned data in real
game code.

## Drills

**D1.** Make this compile by adding lifetime annotations:
```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

**D2.** Explain (in a comment) why this does *not* compile, then fix it by returning an owned
`String` instead of a reference:
```rust
fn make_greeting(name: &str) -> &str {
    let g = format!("Hello, {name}!");
    &g
}
```

**D3.** This struct holds a reference. Add the lifetime parameter so it compiles:
```rust
struct Excerpt {
    part: &str,
}
```
Then construct one from a slice of a `String` in `main`, and print it.

**D4.** Which of these need explicit lifetimes, and which does elision handle? Just answer, then
verify by compiling:
```rust
fn f1(s: &str) -> &str { &s[1..] }
fn f2(a: &str, b: &str) -> usize { a.len() + b.len() }
fn f3(a: &str, b: &str) -> &str { if a.len() > b.len() { a } else { b } }
```

## Mini-project: a borrowing tokenizer

Write a tokenizer that splits a command string into tokens **without allocating** — each token is a
`&str` slice borrowing from the input. This forces you to think about lifetimes (the output borrows
from the input).

```rust
// Return the tokens of `input`, each borrowing from `input`.
fn tokenize<'a>(input: &'a str) -> Vec<&'a str> {
    // split on whitespace, skip empty pieces
    todo!()
}

fn main() {
    let cmd = String::from("  move   3   5  ");
    let tokens = tokenize(&cmd);
    println!("{:?}", tokens);   // ["move", "3", "5"]
    assert_eq!(tokens, vec!["move", "3", "5"]);
}
```

Then write the *owned* alternative `fn tokenize_owned(input: &str) -> Vec<String>` and note the
trade-off: the borrowing version is allocation-free but the result can't outlive `cmd`; the owned
version is independent but copies. **For long-lived game state, prefer owned. For a quick parse
within one function, borrowing is fine.**

**Done when:** both versions work, and you can state in one sentence when you'd choose each.

---
<details><summary>Solution sketch</summary>

```rust
fn tokenize<'a>(input: &'a str) -> Vec<&'a str> {
    input.split_whitespace().collect()   // split_whitespace already skips empties
}
fn tokenize_owned(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}
```
D1: `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`.
D4: f1 elided (one input), f2 no references returned, f3 needs `<'a>` (ambiguous origin).
</details>
