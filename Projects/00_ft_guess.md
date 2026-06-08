```
                        ft_guess
            A warm-up: guess the number
                    Version: 1.0
```

## Foreword

Every systems programmer's first real program reads input, makes a decision, and loops. Before you
build a game *server*, build a game *loop*. `ft_guess` is tiny on purpose — its job is to make the
build/run/edit cycle automatic so you never think about it again.

## Objectives

- Get fluent with `cargo new / run / check`.
- Read user input, parse it safely, handle the error path.
- Use `loop`, `match`, `Option`/`Result` without looking them up.

## General rules

- Language: **Rust**. Build tool: **Cargo**.
- Norm: `cargo fmt` clean, `cargo clippy` with **zero** warnings.
- No `.unwrap()` on user input. Bad input must never crash the program.
- Provide a `Makefile` with targets: `build`, `run`, `lint`, `clean`.

## Mandatory part

Write a CLI game that:

1. Picks a secret number between 1 and 100 (use the `rand` crate: `cargo add rand`).
2. Loops, prompting the player to enter a guess.
3. For each guess, reads a line from stdin and parses it to a number.
   - If the line isn't a valid number, print a friendly message and **loop again** (no crash).
4. Tells the player `too low` / `too high` / `correct`.
5. On a correct guess, prints the number of attempts and exits cleanly.
6. Typing `quit` exits the game.

```
$ cargo run
I'm thinking of a number between 1 and 100.
> 50
Too high!
> abc
That's not a number. Try again.
> 25
Too low!
> 37
Correct! You got it in 3 guesses.
```

## Bonus part

- A difficulty selector (range 1–10 / 1–100 / 1–1000) chosen at startup.
- Limit the number of attempts; lose if exceeded.
- Track and print a best-score (fewest guesses) across rounds within one run.

## Learning objectives (defend these)

- Why does `parse()` return a `Result`, and how did you handle the `Err` arm?
- What's the difference between `io::stdin().read_line(&mut s)` filling a buffer and returning a
  `Result`?
- Where did ownership of the input `String` matter?

## How this feeds TAP

The TAP server is fundamentally "read a line → parse it → act → respond, forever." You just built
the smallest possible version of that loop. Every later project widens it.

➡️ Next rung: `01_ft_inventory.md`
