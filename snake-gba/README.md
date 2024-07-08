# `snake-gba`

This is a simple snake game for the Gameboy Advance. It was written in Rust and leans on the gba crate for GBA-specific functionality.

## Building

`cargo build --release`

## Building to ROM file (.gba)

`./release-rom.sh`

## Running
This project uses the `mgba-qt` emulator. You can run the game with

`cargo run`

Please visit https://mgba.io/ for more information on how to install the emulator on your target platform.

