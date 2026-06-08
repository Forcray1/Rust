# 01 — Basics vs Python & C

This module is a fast tour of syntax, focusing on what's *different*. If you know Python and C,
most of this is recognition, not learning.

## Variables are immutable by default

```rust
let x = 5;        // immutable — cannot reassign
x = 6;            // ❌ compile error!

let mut y = 5;    // `mut` makes it mutable
y = 6;            // ✅ ok
```

- **Python**: everything is rebindable; no concept of const by default.
- **C**: opposite default — mutable unless you write `const`.
- **Rust**: immutable unless you write `mut`. This is a big deal — it makes code easier to reason
  about and is the foundation of safe concurrency later.

## Types: inferred but static

```rust
let a = 5;            // inferred as i32
let b: u64 = 5;       // explicit
let c = 5u8;          // suffix literal
let d = 2.5;          // f64
```

Static like C, but inferred like... well, better than C. You rarely write types for locals,
but **function signatures always need types** (like C, unlike Python).

### The integer types (this matters for packets/networking)

| Rust | Meaning | C equivalent |
|------|---------|--------------|
| `i8 i16 i32 i64 i128` | signed | `int8_t`… |
| `u8 u16 u32 u64 u128` | unsigned | `uint8_t`… |
| `usize` / `isize` | pointer-sized (array indices) | `size_t` / `ssize_t` |
| `f32` / `f64` | floats | `float` / `double` |
| `bool` | true/false | `bool` |
| `char` | a **Unicode scalar** (4 bytes!) | not like C's `char` |

> Note: Rust's `char` is 4 bytes (a full Unicode codepoint), NOT a byte. A byte is `u8`.
> This bites C programmers. Strings are UTF-8 (more below).

Integer overflow: **panics in debug builds**, wraps in release. Use `wrapping_add`,
`checked_add`, `saturating_add` when you want explicit behavior — important for game math.

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b           // no `return`, no semicolon → this is the return VALUE
}
```

Key idea: **the last expression without a semicolon is the return value.** This "expression-
oriented" style is everywhere. A semicolon turns an expression into a statement (value discarded).

```rust
let x = {
    let a = 2;
    a * 10          // block evaluates to 20
};                  // x == 20
```

## Control flow is expression-based

```rust
let grade = if score > 90 { "A" } else { "B" };   // if returns a value (like Python's ternary, but general)
```

```rust
loop { ... break; }              // infinite loop
while condition { ... }
for i in 0..10 { ... }           // range 0..10 is exclusive; 0..=10 is inclusive
for item in &collection { ... }  // iterate by reference
```

There is no C-style `for(i=0;i<n;i++)`. You use ranges and iterators. `loop` can even return a value:

```rust
let result = loop {
    break 42;        // loop evaluates to 42
};
```

## Strings: two types (this confuses everyone at first)

```rust
let a: &str = "hello";        // string SLICE — borrowed, fixed, usually a literal
let b: String = String::from("hello");  // owned, growable, heap-allocated
```

- `&str` ≈ a *view* into string data (think C's `const char*` but length-aware and UTF-8-safe).
- `String` ≈ Python's `str` / C++'s `std::string` — owns its buffer, can grow.

You'll pass `&str` to functions (cheap, borrowed) and build/own with `String`. The relationship
is the same as `&[T]` (slice) vs `Vec<T>` (owned vector). This clicks fully after module 02.

```rust
let mut s = String::new();
s.push_str("foo");
s.push('!');
let combined = format!("{s} and more");   // format! is like Python f-strings
```

## Printing & formatting

```rust
println!("x = {x}");                 // inline variable (like f-string)
println!("x = {}, y = {}", x, y);    // positional
println!("{:?}", some_struct);       // debug format (need #[derive(Debug)])
println!("{:#?}", some_struct);      // pretty debug
eprintln!("error!");                 // to stderr
```

## No null, no exceptions

- **No `null` / `None`-as-a-footgun**: absence is modeled with `Option<T>` (module 04).
- **No exceptions**: errors are values via `Result<T, E>` (module 04). There's `panic!` for
  unrecoverable bugs, but you design around `Result`, not try/except.

## Comments & docs

```rust
// line comment
/* block */
/// doc comment for the item below (renders to HTML with `cargo doc`)
```

## The one big thing this module did NOT cover

Everything above is the "easy 20%". The hard, defining 80% of Rust is **ownership** — how Rust
manages memory with no GC and no manual `free`, enforced at compile time. That's next, and it's
the concept that makes Rust *Rust*.

---

➡️ Next: `02_ownership_and_borrowing.md` ⭐ (the most important module)
