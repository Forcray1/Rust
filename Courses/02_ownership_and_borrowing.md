# 02 — Ownership & Borrowing ⭐

**This is the module.** Ownership is the one idea that doesn't exist in Python or C. Master it and
the rest of Rust is downhill. Fight it and you'll suffer. Read this twice.

## The problem ownership solves

How do you free memory at the right time?

- **Python**: a garbage collector tracks references and frees things eventually. Cost: runtime
  overhead, pauses, unpredictable timing — bad for a game loop.
- **C**: *you* call `free()`. Cost: forget it → leak; do it twice → crash; use after free →
  security hole. Entire classes of bugs (use-after-free, double-free, dangling pointers) live here.
- **Rust**: the *compiler* figures out when to free, by tracking **ownership** at compile time.
  Zero runtime cost, zero GC, and the bugs above literally cannot compile.

## Rule 1: every value has exactly one owner

```rust
let s = String::from("hello");   // `s` owns the heap buffer
```

When the owner goes out of scope, the value is dropped (freed) automatically:

```rust
{
    let s = String::from("hello");
    // use s
}   // <- s goes out of scope here; its memory is freed automatically. No free() call.
```

This is like C++'s RAII / destructors, applied to everything.

## Rule 2: assigning/passing MOVES ownership (for non-Copy types)

This is the shocker for Python *and* C people:

```rust
let s1 = String::from("hello");
let s2 = s1;            // ownership MOVES from s1 to s2
println!("{s1}");      // ❌ ERROR: value borrowed after move
```

After `let s2 = s1;`, `s1` is *invalid*. There's only one owner, and it's now `s2`. This prevents
two variables from both thinking they should free the same buffer (the C double-free bug).

- In **Python**, `s2 = s1` makes both names point to the same object (shared reference).
- In **C**, `s2 = s1` copies the pointer — now two pointers to one buffer (danger).
- In **Rust**, the old name is *deactivated*. Safe by construction.

The same happens passing to a function:

```rust
fn consume(s: String) { /* s owned here */ }   // s is dropped at end

let name = String::from("Alice");
consume(name);
println!("{name}");    // ❌ ERROR: name was moved into consume()
```

### Copy types are the exception

Small, stack-only types (`i32`, `bool`, `char`, `f64`, tuples of these…) implement the `Copy`
trait — they're *copied*, not moved, because copying a few bytes is trivial:

```rust
let x = 5;
let y = x;        // x is COPIED
println!("{x}");  // ✅ fine — x still valid
```

Rule of thumb: **stack-only fixed-size data is Copy; anything owning heap memory (String, Vec…)
is move-only.**

## Borrowing: references let you use without owning

Moving everything around is painful. The fix is **borrowing** — taking a *reference* (`&`) to a
value without taking ownership:

```rust
fn length(s: &String) -> usize {   // borrows; does NOT take ownership
    s.len()
}

let name = String::from("Alice");
let n = length(&name);   // lend a reference
println!("{name}: {n}"); // ✅ name still valid — it was only borrowed
```

`&name` = "a reference to name". Like a C pointer, but the compiler guarantees it's always valid
(never dangling, never null).

## Rule 3: the borrowing rules (the heart of it)

At any given time, you can have **either**:

- **any number of immutable references** (`&T`), **OR**
- **exactly one mutable reference** (`&mut T`),

…but **not both at once**.

```rust
let mut s = String::from("hi");

let r1 = &s;        // immutable borrow
let r2 = &s;        // another immutable borrow — fine, many readers OK
println!("{r1} {r2}");

let m = &mut s;     // mutable borrow — OK now (r1/r2 no longer used)
m.push_str(" there");
```

But you cannot mix:

```rust
let mut s = String::from("hi");
let r = &s;          // immutable borrow
let m = &mut s;      // ❌ ERROR: cannot borrow `s` as mutable while borrowed as immutable
println!("{r}");
```

### Why this rule exists (and why it's genius)

"Many readers XOR one writer" is exactly the rule that prevents **data races**. If no one can
write while others read, you can't observe half-updated data. Rust enforces this *at compile
time, single-threaded too* — and the same rule makes multithreading safe for free (module 10).

It also prevents subtle bugs like iterator invalidation:

```rust
let mut v = vec![1, 2, 3];
for x in &v {            // immutable borrow of v for the loop
    v.push(*x);          // ❌ ERROR — can't mutate v while iterating it
}
```

In C++ this compiles and corrupts memory. In Python it raises (sometimes) at runtime. In Rust it
*won't build*.

## Mutable references

```rust
fn add_exclamation(s: &mut String) {
    s.push('!');
}

let mut greeting = String::from("hi");
add_exclamation(&mut greeting);   // lend a mutable reference
println!("{greeting}");           // "hi!"
```

Note `mut` appears twice: the variable must be `mut`, and you take `&mut`.

## No dangling references — guaranteed

```rust
fn dangle() -> &String {        // ❌ won't compile
    let s = String::from("oops");
    &s                          // returning a reference to s, but s is freed here!
}
```

The compiler refuses because the reference would outlive the data. In C this is the classic
"return pointer to local variable" use-after-free. Rust catches it at compile time. (The mechanism
that tracks this is **lifetimes**, module 07.)

## Slices: borrowing part of a collection

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];       // a slice — a borrowed view, no copy
let v = vec![1, 2, 3, 4, 5];
let middle: &[i32] = &v[1..4];    // [2, 3, 4]
```

A slice is a (pointer, length) pair into existing data. This is why functions usually take `&str`
and `&[T]` — they work on both owned and borrowed data without copying.

## Clone: opt out explicitly when you really need a copy

```rust
let s1 = String::from("hello");
let s2 = s1.clone();    // deep copy — now two independent owners
println!("{s1} {s2}");  // both valid
```

`.clone()` is always explicit and visible. This is intentional: copies can be expensive, so Rust
makes you *ask*. When learning, it's fine to `.clone()` to get unstuck — but each clone you remove
later is a small performance win. **For an MMORPG, gratuitous cloning of world state is exactly
the thing you'll learn to avoid.**

## How to think about it (the mental model)

> Ownership = "who is responsible for cleaning this up."
> Borrowing = "let me look at / temporarily modify your thing, I promise to give it back and not
> hold onto it longer than you have it."

When the borrow checker yells at you, ask: *"Who owns this? How long does each reference live? Am
I trying to write while someone's reading?"* The answer is usually right there.

## Common beginner reactions (all normal)

- "The compiler hates me." → It's teaching you. Read the full error; it often suggests the fix.
- "I'll just `.clone()` everything." → Fine to start. Refactor away clones as you understand more.
- "Why won't it let me do X?" → Usually because X would be a memory/data-race bug in C. That's the
  point.

---

🏋️ Do `Exercises/02_ownership.md` now — this concept only sticks by fighting the compiler.

➡️ Next: `03_structs_enums_pattern_matching.md`
