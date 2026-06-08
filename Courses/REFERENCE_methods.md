# Methods Reference / Cheatsheet

A practical lookup of the methods you'll use most, grouped by type. Not exhaustive — these are the
everyday ones. The full truth is always at https://doc.rust-lang.org/std/ (and `cargo doc --open`).

**How to read this:** in Rust you call methods *on* a value with a dot — `thing.method(args)` —
not as a global function. Most of these return either a new value (transform) or a `bool`/number
(query), and many that can fail return an `Option` or `Result` (see those sections).

Quick mental tags used below:
- 🔄 returns a **new** value (original untouched unless it's `&mut self`)
- ✏️ **mutates** in place (the variable must be `mut`)
- ❓ returns an **`Option`** (might be nothing)
- 🟰 returns a **`Result`** (might be an error)

---

## `String` and `&str` (text)

`String` = owned, growable text. `&str` = a borrowed view (slice) of text. Most *reading* methods
exist on both; *growing/mutating* methods need an owned, `mut` `String`.

### Create
| Method | Does | Example |
|--------|------|---------|
| `String::new()` | empty owned string | `let mut s = String::new();` |
| `String::from("x")` | owned string from a literal | `String::from("hi")` |
| `"x".to_string()` | same, via method | `"hi".to_string()` |
| `"x".to_owned()` | `&str` → owned `String` | `"hi".to_owned()` |
| `format!("{a} {b}")` | build a string like an f-string 🔄 | `format!("{name}: {hp}")` |

### Append / modify (need `let mut`)
| Method | Does | Example |
|--------|------|---------|
| `.push_str("x")` | ✏️ append a **string** to the end | `s.push_str(" world")` |
| `.push('c')` | ✏️ append a single **char** | `s.push('!')` |
| `+` operator | 🔄 concatenate (consumes left, borrows right) | `let s3 = s1 + &s2;` |
| `.insert_str(i, "x")` | ✏️ insert at byte index | `s.insert_str(0, "Mr. ")` |
| `.clear()` | ✏️ empty it | `s.clear()` |
| `.truncate(n)` | ✏️ keep first `n` bytes | `s.truncate(5)` |
| `.pop()` | ✏️❓ remove & return last char | `s.pop()` |

> **To append to a string** (your question): `.push_str(...)` for text, `.push(...)` for one char,
> `format!(...)` to build a fresh one, or `+` to concatenate two.

### Query / inspect (work on `&str` too)
| Method | Does | Example |
|--------|------|---------|
| `.len()` | number of **bytes** (not chars!) | `s.len()` |
| `.is_empty()` | true if length 0 | `s.is_empty()` |
| `.contains("x")` | substring present? | `s.contains("err")` |
| `.starts_with("x")` / `.ends_with("x")` | prefix / suffix check | `s.starts_with("OK")` |
| `.find("x")` | ❓ byte index of first match | `s.find(' ')` |
| `.chars().count()` | number of **characters** | `"café".chars().count()` → 4 |

### Transform (🔄 return a new value)
| Method | Does | Example |
|--------|------|---------|
| `.to_uppercase()` / `.to_lowercase()` | change case | `s.to_lowercase()` |
| `.trim()` | strip leading/trailing whitespace | `"  hi  ".trim()` → `"hi"` |
| `.replace("a", "b")` | replace all occurrences | `s.replace("_", " ")` |
| `.repeat(n)` | repeat the string `n` times | `"ab".repeat(3)` → `"ababab"` |

### Split / parse (huge for the protocol project)
| Method | Does | Example |
|--------|------|---------|
| `.split(' ')` | iterator of pieces by separator | `"a b c".split(' ')` |
| `.split_whitespace()` | split on any whitespace, skip empties | `"  a  b ".split_whitespace()` |
| `.splitn(n, ' ')` | split into at most `n` pieces | `"cmd a b".splitn(2, ' ')` |
| `.split_once(' ')` | ❓ split into (before, after) at first sep | `"MOVE north".split_once(' ')` |
| `.lines()` | iterator over lines | `text.lines()` |
| `.parse::<T>()` | 🟰 parse text into a number/type | `"42".parse::<u32>()` |
| `.chars()` | iterator of characters | `"hi".chars()` |
| `.bytes()` | iterator of bytes (`u8`) | `"hi".bytes()` |
| `.as_bytes()` | view as `&[u8]` | `s.as_bytes()` |
| `.as_str()` | `String` → `&str` | `s.as_str()` |

```rust
// Typical command parse (project ft_protocol):
let line = "MOVE north";
if let Some((verb, arg)) = line.split_once(' ') {
    // verb = "MOVE", arg = "north"
}
let n: u32 = "42".parse().unwrap_or(0);   // parse returns Result → handle it
```

---

## `Vec<T>` (growable list)

Owned, heap-allocated, growable array. Needs `mut` for anything that changes it.

### Create
| Method | Does | Example |
|--------|------|---------|
| `Vec::new()` | empty vec | `let mut v = Vec::new();` |
| `vec![a, b, c]` | vec with elements (macro) | `vec![1, 2, 3]` |
| `vec![x; n]` | `x` repeated `n` times | `vec![0; 5]` |
| `.collect()` | build from an iterator 🔄 | `(0..5).collect::<Vec<_>>()` |

### Add / remove (need `mut`)
| Method | Does | Example |
|--------|------|---------|
| `.push(x)` | ✏️ add to the **end** (≈ Python `.append`) | `v.push(4)` |
| `.pop()` | ✏️❓ remove & return last | `v.pop()` |
| `.insert(i, x)` | ✏️ insert at index `i` | `v.insert(0, 9)` |
| `.remove(i)` | ✏️ remove & return at index `i` | `v.remove(2)` |
| `.retain(\|x\| cond)` | ✏️ keep only elements matching | `v.retain(\|&x\| x > 0)` |
| `.clear()` | ✏️ remove all | `v.clear()` |
| `.extend(other)` | ✏️ append all of another iterable | `v.extend(vec![7, 8])` |
| `.dedup()` | ✏️ remove consecutive duplicates | `v.dedup()` |
| `.sort()` | ✏️ sort ascending | `v.sort()` |
| `.sort_by_key(\|x\| k)` | ✏️ sort by a key | `v.sort_by_key(\|p\| p.hp)` |
| `.reverse()` | ✏️ reverse in place | `v.reverse()` |

### Access / query
| Method | Does | Example |
|--------|------|---------|
| `v[i]` | element at `i` (**panics** if out of range) | `v[0]` |
| `.get(i)` | ❓ safe access, no panic | `v.get(5)` |
| `.first()` / `.last()` | ❓ first / last element | `v.first()` |
| `.len()` | number of elements | `v.len()` |
| `.is_empty()` | true if empty | `v.is_empty()` |
| `.contains(&x)` | does it contain `x`? | `v.contains(&3)` |
| `.iter()` | iterator of `&T` (read) | `v.iter()` |
| `.iter_mut()` | iterator of `&mut T` (modify) | `v.iter_mut()` |
| `.into_iter()` | iterator of `T` (consumes vec) | `v.into_iter()` |

---

## `HashMap<K, V>` (dictionary)

`use std::collections::HashMap;` — key→value store, like a Python `dict`.

| Method | Does | Example |
|--------|------|---------|
| `HashMap::new()` | empty map | `let mut m = HashMap::new();` |
| `.insert(k, v)` | ✏️❓ set key; returns old value if any | `m.insert(1, player)` |
| `.get(&k)` | ❓ get `&V` for a key | `m.get(&1)` |
| `.get_mut(&k)` | ❓ get `&mut V` (to modify in place) | `m.get_mut(&1)` |
| `.remove(&k)` | ✏️❓ remove & return value | `m.remove(&1)` |
| `.contains_key(&k)` | key present? | `m.contains_key(&1)` |
| `.entry(k).or_insert(v)` | get existing or insert default | see below |
| `.entry(k).or_insert_with(\|\| v)` | same, value built lazily | `m.entry(k).or_insert_with(Vec::new)` |
| `.len()` / `.is_empty()` | size / emptiness | `m.len()` |
| `.keys()` / `.values()` / `.values_mut()` | iterate keys / values | `for v in m.values()` |
| `.iter()` | iterate `(&K, &V)` pairs | `for (k, v) in &m` |

```rust
// "get or create" — the most useful map pattern:
let mut counts: HashMap<&str, u32> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;   // count occurrences
}
```

## `HashSet<T>` (unique values)
| Method | Does | Example |
|--------|------|---------|
| `.insert(x)` | ✏️ add; returns false if already present | `set.insert(3)` |
| `.contains(&x)` | membership test | `set.contains(&3)` |
| `.remove(&x)` | ✏️ remove | `set.remove(&3)` |

---

## `Option<T>` (a value, or nothing)

Replaces null. `Some(v)` or `None`. (Course 04.)

| Method | Does | Example |
|--------|------|---------|
| `.is_some()` / `.is_none()` | which variant? | `o.is_some()` |
| `.unwrap()` | get value, **panics** on `None` (avoid in real code) | `o.unwrap()` |
| `.expect("msg")` | like unwrap with a custom panic message | `o.expect("must exist")` |
| `.unwrap_or(default)` | value, or a fallback | `o.unwrap_or(0)` |
| `.unwrap_or_else(\|\| ...)` | value, or compute a fallback | `o.unwrap_or_else(make_default)` |
| `.unwrap_or_default()` | value, or the type's `Default` | `o.unwrap_or_default()` |
| `.map(\|v\| ...)` | 🔄 transform the inner value if present | `o.map(\|v\| v * 2)` |
| `.and_then(\|v\| ...)` | chain another Option-returning op | `o.and_then(parse)` |
| `.filter(\|v\| cond)` | keep `Some` only if condition holds | `o.filter(\|&v\| v > 0)` |
| `.ok_or(err)` | 🟰 `Option` → `Result` | `o.ok_or("missing")` |
| `if let Some(v) = o` | destructure one case | see below |

```rust
if let Some(player) = world.get(&id) {
    println!("{}", player.name);
}
let hp = maybe_hp.unwrap_or(100);   // default when absent
```

---

## `Result<T, E>` (success, or error)

Replaces exceptions. `Ok(v)` or `Err(e)`. (Course 04.)

| Method | Does | Example |
|--------|------|---------|
| `.is_ok()` / `.is_err()` | which variant? | `r.is_ok()` |
| `?` (operator) | unwrap `Ok`, or return the `Err` early | `let x = thing()?;` |
| `.unwrap()` | value, **panics** on `Err` | `r.unwrap()` |
| `.expect("msg")` | unwrap with a message | `r.expect("parse failed")` |
| `.unwrap_or(default)` | value, or fallback | `r.unwrap_or(0)` |
| `.ok()` | ❓ `Result` → `Option` (discard error) | `"42".parse().ok()` |
| `.map(\|v\| ...)` | 🔄 transform the `Ok` value | `r.map(\|v\| v + 1)` |
| `.map_err(\|e\| ...)` | transform the error | `r.map_err(to_my_error)` |
| `match r { Ok(v) => .., Err(e) => .. }` | handle both | see below |

```rust
match "42".parse::<u32>() {
    Ok(n)  => println!("got {n}"),
    Err(e) => println!("bad number: {e}"),
}
```

---

## Iterators (the chaining tools)

You get an iterator from `.iter()`, `.iter_mut()`, `.into_iter()`, `.chars()`, ranges (`0..n`),
etc., then chain **adapters** (lazy) and finish with a **consumer**. (Course 06.)

### Adapters (🔄 lazy — return another iterator)
| Method | Does | Example |
|--------|------|---------|
| `.map(\|x\| ...)` | transform each item | `.map(\|x\| x * 2)` |
| `.filter(\|x\| cond)` | keep matching items | `.filter(\|&x\| x > 0)` |
| `.filter_map(\|x\| opt)` | map + drop `None`s | `.filter_map(\|s\| s.parse().ok())` |
| `.enumerate()` | yield `(index, item)` | `.enumerate()` |
| `.zip(other)` | pair two iterators | `a.iter().zip(b.iter())` |
| `.take(n)` / `.skip(n)` | first `n` / drop first `n` | `.take(5)` |
| `.rev()` | reverse | `.rev()` |
| `.chain(other)` | concatenate iterators | `a.chain(b)` |
| `.flatten()` | flatten nested iterators | `.flatten()` |

### Consumers (drive the chain, produce a final value)
| Method | Does | Example |
|--------|------|---------|
| `.collect()` | gather into `Vec`/`String`/`HashMap`… 🔄 | `.collect::<Vec<_>>()` |
| `.count()` | how many items | `.count()` |
| `.sum()` / `.product()` | numeric reduce | `.sum::<i32>()` |
| `.min()` / `.max()` | ❓ smallest / largest | `.max()` |
| `.min_by_key(\|x\| k)` / `.max_by_key` | ❓ by a key | `.max_by_key(\|p\| p.hp)` |
| `.find(\|x\| cond)` | ❓ first match | `.find(\|&x\| x > 10)` |
| `.position(\|x\| cond)` | ❓ index of first match | `.position(\|&x\| x == 5)` |
| `.any(\|x\| cond)` | any match? (bool) | `.any(\|p\| !p.is_alive())` |
| `.all(\|x\| cond)` | all match? (bool) | `.all(\|&x\| x > 0)` |
| `.fold(init, \|acc, x\| ...)` | general reduce | `.fold(0, \|a, x\| a + x)` |
| `.for_each(\|x\| ...)` | side effect per item | `.for_each(\|x\| println!("{x}"))` |

```rust
// Names of living players within range — one pass:
let nearby: Vec<&str> = players.values()
    .filter(|p| p.is_alive())
    .filter(|p| distance(p.pos, center) < 10.0)
    .map(|p| p.name.as_str())
    .collect();
```

---

## Numbers (`i32`, `u32`, `f32`, `usize`, …)

| Method | Does | Example |
|--------|------|---------|
| `.min(other)` / `.max(other)` | clamp to a bound | `(hp + 5).min(100)` |
| `.clamp(lo, hi)` | clamp into a range | `x.clamp(0, 100)` |
| `.abs()` | absolute value | `(-3).abs()` |
| `.pow(n)` | integer power | `2u32.pow(10)` |
| `.powi(n)` / `.powf(x)` | float power | `2.0f32.powi(3)` |
| `.sqrt()` | square root (floats) | `16.0f32.sqrt()` |
| `.saturating_sub(n)` | subtract, floor at 0 (no underflow) | `hp.saturating_sub(dmg)` |
| `.saturating_add(n)` | add, cap at max | `hp.saturating_add(heal)` |
| `.checked_add(n)` | ❓ add, `None` on overflow | `a.checked_add(b)` |
| `.wrapping_add(n)` | add, wraps around on overflow | `a.wrapping_add(b)` |
| `as u32` | cast between number types | `x as f32` |
| `.to_string()` | number → text | `42.to_string()` |

> **`saturating_sub` is your friend for HP/combat** (project ft_inventory/ft_adventure): it subtracts
> without ever underflowing below 0, so a player's HP can't wrap around to a huge number.

---

## `char` (a single Unicode character)

| Method | Does | Example |
|--------|------|---------|
| `.is_alphabetic()` | letter? | `c.is_alphabetic()` |
| `.is_numeric()` / `.is_ascii_digit()` | digit? | `c.is_ascii_digit()` |
| `.is_whitespace()` | space/tab/newline? | `c.is_whitespace()` |
| `.to_uppercase()` / `.to_lowercase()` | change case | `c.to_ascii_uppercase()` |
| `.to_digit(10)` | ❓ char → number | `'7'.to_digit(10)` → `Some(7)` |

---

## Common conversions (who turns into what)

| From → To | How |
|-----------|-----|
| `&str` → `String` | `.to_string()` / `.to_owned()` / `String::from(s)` |
| `String` → `&str` | `.as_str()` / `&s` (auto-deref in many places) |
| number → `String` | `.to_string()` / `format!("{n}")` |
| `&str` → number | `.parse::<u32>()` (returns `Result`) |
| `Vec<T>` → `&[T]` (slice) | `&v` / `v.as_slice()` |
| iterator → `Vec`/`String`/`HashMap` | `.collect()` |
| `Option` → `Result` | `.ok_or(err)` |
| `Result` → `Option` | `.ok()` |

---

## How to find more

- In your editor (rust-analyzer): type `value.` and autocomplete lists every method, with docs.
- `cargo doc --open` builds searchable docs for your project + dependencies.
- Online: https://doc.rust-lang.org/std/ — search a type (e.g. "Vec") to see *all* its methods.

When you meet a new method in the courses/projects, add a row here. Your own cheatsheet beats any
generic one.
