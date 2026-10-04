# Snake and Ladders — Terminal Version

This is the first version of the project. It is a two-player terminal game built with core Rust concepts only; it does not use a graphics library.

## Run it

```bash
cargo run --bin snakes_and_ladders
```

## What it demonstrates

- A `Player` struct to store each name, symbol, and position.
- Arrays for players, ladders, and snakes.
- Loops to control turns and print the 10×10 board.
- Functions for dice rolls, movement, rule checks, board drawing, and input.
- `if`/`else` conditions for snakes, ladders, exact-win rules, and token display.

## Project progression

1. **Terminal version:** focuses on game rules and Rust fundamentals.
2. **Graphical version:** keeps the same game idea and adds Macroquad for the visual board, buttons, tokens, and dice.
