# Exercises 02 — Ownership & Borrowing

The most important reps you'll do. The goal isn't clever code — it's making the borrow checker
your friend.

## Drills

**D1. Predict the error.** Without running, say why each fails, then run to confirm:

```rust
// (a)
let s = String::from("hi");
let s2 = s;
println!("{s}");

// (b)
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{first}");

// (c)
let mut s = String::from("hi");
let r1 = &mut s;
let r2 = &mut s;
println!("{r1} {r2}");
```

**D2. Fix each** of the above so it compiles, in the *cheapest* way (avoid `.clone()` if a borrow
works).

**D3. Borrow vs move in functions.** Write three functions over a `String`:
- `fn inspect(s: &String) -> usize` — returns its length, caller keeps the string.
- `fn shout(s: &mut String)` — appends `"!"`.
- `fn consume(s: String)` — takes ownership and prints it (string unusable afterward).
Call all three from `main` in an order that compiles. Why must `consume` be called last?

**D4. Slices.** Write `fn first_word(s: &str) -> &str` that returns the first whitespace-delimited
word (a slice, no allocation). Test on `"hello world"` → `"hello"`.

## Mini-project: the borrow-checker gauntlet

Below is a deliberately broken program. **Fix every error so it compiles and prints the expected
output, changing as little as possible.** Decide for each fix whether you need a borrow, a `mut`, a
`clone`, or a restructure.

```rust
struct Inventory {
    items: Vec<String>,
}

fn add_item(inv: Inventory, item: String) {
    inv.items.push(item);
}

fn describe(inv: Inventory) -> String {
    format!("{} items", inv.items.len())
}

fn main() {
    let inv = Inventory { items: vec![] };
    add_item(inv, String::from("sword"));
    add_item(inv, String::from("shield"));
    println!("{}", describe(inv));
    println!("{}", describe(inv));   // expected: "2 items" twice
}
```

**Done when:** it compiles, prints `2 items` twice, and you can explain in one sentence why the
original moved-ownership version couldn't work.

---
<details><summary>Solution sketch</summary>

- `add_item` must take `&mut Inventory` and the variable `inv` must be `mut`.
- `describe` must take `&Inventory` (borrow), so it can be called twice.

```rust
fn add_item(inv: &mut Inventory, item: String) { inv.items.push(item); }
fn describe(inv: &Inventory) -> String { format!("{} items", inv.items.len()) }

fn main() {
    let mut inv = Inventory { items: vec![] };
    add_item(&mut inv, String::from("sword"));
    add_item(&mut inv, String::from("shield"));
    println!("{}", describe(&inv));
    println!("{}", describe(&inv));
}
```

D1 answers: (a) `s` moved into `s2`; (b) can't mutate `v` while `first` borrows it (push may
reallocate, invalidating the reference — the bug Rust prevents); (c) two `&mut` to the same value
at once is forbidden.

D4:
```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') { Some(i) => &s[..i], None => s }
}
```
</details>
