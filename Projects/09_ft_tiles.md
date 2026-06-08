```
                       ft_tiles
   A 2D tile-map GUI client — Pokémon Black & White style
                    Version: 1.0
```

## Foreword

TAP only asks for *a* GUI. Your group's ambition is bigger: a top-down, tile-based world you walk
around like Pokémon Black & White, with the chat/inventory/log panels TAP requires layered on top.
This project gets you there in two halves you can tackle separately:

- **Half A — the renderer:** draw a tile map, move a character sprite around a grid, with a camera.
- **Half B — the TAP GUI:** wire that renderer to your project-07 server and add the required UI
  panels (room view, inventory with TAKE/DROP buttons, separated chat/log views, action buttons,
  player counters, NPC dialogue).

Do Half A as a standalone toy first. Only then connect the network.

## Toolkit choice (recommended, justify in README)

- **`macroquad`** — minimal-friction 2D: load a texture atlas, `draw_texture_ex` for tiles/sprites,
  built-in input and camera. Easiest route to "walk on a grid." `cargo add macroquad`.
- **`egui`** via **`egui-macroquad`** — immediate-mode panels/buttons for inventory, chat, actions.
  `cargo add egui-macroquad`.
- Alternative for later growth: **`bevy`** (full ECS engine — ties back to Course 14). More power,
  steeper curve. Start with macroquad; migrate to bevy only if the game outgrows it.

## General rules

- Rust + Cargo. Norm clean.
- Reuse the `protocol` crate (02). The networking runs alongside the render loop — note that
  macroquad owns the main loop, so run the TCP client on a background thread/task and communicate
  via a channel (course 10/11). Document this bridge.
- `Makefile`: `install`, `build`, `run-client-gui`, `lint`, `clean`.

## Mandatory part

### Half A — tile renderer (offline)

1. **Tile map:** represent a room/area as a 2D grid of tile ids; draw it from a tile sheet
   (grass, path, wall, water…). A simple `Vec<Vec<TileId>>` or a flat `Vec<TileId>` + width.
2. **Character sprite:** draw the player on the grid; move one cell per key press (arrow/WASD) with
   collision against non-walkable tiles. B/W-style grid movement (snap to cells) is fine — no need
   for smooth pixel movement at first (that's a bonus).
3. **Camera:** keep the player centered (or clamp at edges) as they walk a map larger than the
   screen.

### Half B — TAP GUI client

4. **Connect** to the project-07 server (`CONNECT <name>`), running networking off the render
   thread; feed server events into the render loop via a channel.
5. **Room view:** render the current room from `LOOK`/state — its tiles, items on the ground, NPCs,
   other players, and the available exits. Update **in real time** as `EVT`s arrive (someone enters,
   an item is taken, etc.).
6. **Inventory panel:** show `INVENTORY`; provide **TAKE** and **DROP** buttons (handling item id
   *or* display name). The room view must auto-update after TAKE/DROP to reflect availability.
7. **Action buttons:** for `LOOK`, `MOVE`, `TAKE`, `DROP`, `TALK`, `ATTACK`, `STATUS`, `QUEST`,
   `QUESTS`, `WHO`, `GROUP`, `QUIT`.
8. **Separated views:** a **chat view** (Global / Room / Group tabs or sections) kept distinct from
   a **log view**.
9. **Counters:** players in the current room and players on the server (`WHO`).
10. **NPC interaction:** `TALK` opens a dialogue display.
11. **Responsiveness:** the UI never freezes while events stream in.

## Bonus part

- Smooth (tweened) movement between cells, walk animation frames — true B/W feel.
- Animated NPCs / other players moving as their presence events arrive.
- A minimap, day/night tint, simple audio.
- A tiled map editor, or import from the Tiled (`.tmx`) format.

## Learning objectives (defend these)

- macroquad's `main` *is* the loop — how do you run a TCP client without blocking rendering? Draw
  the thread/channel diagram. (This is the GUI version of "stay responsive while receiving.")
- How does the GUI stay a *thin view* over server-authoritative state? What happens if the client
  and server disagree — who wins, and why must it be the server?
- Map each TAP GUI requirement above to the widget/draw call that satisfies it.

## How this feeds TAP

This is TAP's **GUI client** *and* your 2D Pokémon-style ambition in one. Because it speaks the
shared `protocol`, it interoperates with any group's server. With projects 07 + 08 + 09 you now hold
all three TAP deliverables.

➡️ Final rung: `10_TAP_final.md` — assemble and ship.
