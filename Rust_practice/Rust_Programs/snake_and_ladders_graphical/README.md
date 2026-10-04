# Graphical Snake and Ladders

A separate graphical version of Snake and Ladders built with Rust and Macroquad. The terminal version remains in the parent Rust practice project.

## Run on NixOS

```bash
cd snake_and_ladders_graphical
nix-shell
cargo run
```

The first `nix-shell` run downloads the graphical libraries. You can press `Space` or click **ROLL DICE** to play, click **PLAY AGAIN** after a win, and press `Esc` to close the window.

## What it contains

- A visual 10×10 zig-zag board
- Red and Blue player tokens
- Snakes, ladders, dice, turn display, and restart button
- The same exact-roll-to-100 game rule as the terminal version
