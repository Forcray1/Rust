```
                      ft_inventory
        An RPG character & item manager (CLI)
                    Version: 1.0
```

## Foreword

TAP needs an item system with real rules: items are unique instances, taking one removes it,
dropping makes it available again, and players have HP that combat chips away. That's a *data
modeling* problem before it's a networking one. Here you model it offline, with no sockets in the
way, so the design is clean before the wire complicates it.

## Objectives

- Model game entities with `struct`, `enum`, and `impl`.
- Use `Vec` and `HashMap` to hold world/inventory state.
- Express shared behavior with a `trait`.
- Respect ownership: no accidental duplication of "unique" items.

## General rules

- Rust + Cargo. Norm: `cargo fmt` + `cargo clippy` clean.
- `Makefile`: `build`, `run`, `lint`, `clean`.

## Mandatory part

Build a CLI program that simulates a single character interacting with items and simple combat.

1. **Item model.** An `Item` has: a unique `id` (e.g. `item.herbs`), a display `name` (multi-word
   allowed, e.g. `"Healing Herbs"`), and a `kind` enum (`Weapon { damage }`, `Potion { heal }`,
   `Misc`). Items are **unique instances** — the same item can be in exactly one place at a time.
2. **Player model.** A `Player` has a name, `hp` (start 100, max 100), and an inventory
   (`Vec<Item>` or `HashMap<String, Item>`).
3. **A "room"/floor** holding a set of items on the ground.
4. **A command REPL** supporting at least:
   - `look` — list items on the ground and the player's HP.
   - `take <id|name>` — move an item from the ground into the inventory (by id **or** display
     name; multi-word names must work). Removing from the ground must not leave a copy behind.
   - `drop <id|name>` — move an item from inventory back to the ground.
   - `inventory` — list held items.
   - `use <id|name>` — apply a potion (heal, capped at 100) or report a weapon can't be "used".
   - `quit`.
5. **No duplication, ever.** Taking then dropping then taking the same item must always be the
   *same single instance*. Prove this in your README with a short transcript.

```
> look
On the ground: Healing Herbs (item.herbs), Rusty Sword (item.sword)
HP: 100/100
> take herbs
Took Healing Herbs.
> use herbs
You feel refreshed. (HP unchanged — already full)
> take Rusty Sword
Took Rusty Sword.
> inventory
- Rusty Sword (item.sword)
```

## Bonus part

- Add a `Monster` with HP and an `attack <target>` command using a damage formula you design
  (document it).
- Equip a weapon so `attack` uses its `damage`.
- Multiple rooms you can `move` between, each with its own ground items (preview of `ft_world`).

## Learning objectives (defend these)

- How does Rust's ownership model *guarantee* an item can't be duplicated? Where would a careless
  `.clone()` have created a bug?
- Why did you choose `Vec` vs `HashMap` for the inventory? What does each cost for `take`/`drop`?
- How does the `trait` (e.g. `Usable`) let `look`/`use` treat different item kinds uniformly?

## How this feeds TAP

This is TAP's **Dynamic Item Management** and the seed of its **Combat System**, built in isolation
where you can get the rules exactly right. You'll lift this model almost verbatim into the server.

➡️ Next rung: `02_ft_protocol.md`
