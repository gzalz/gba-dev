# `snake-gba`

This is a simple snake game for the Gameboy Advance. It was written in Rust and leans on the gba crate for GBA-specific functionality.

## Running Pre-Compiled ROM

A pre-compiled ROM, `snake.gba`, is available in the `rom` directory. You can run this ROM on your GBA emulator or hardware.

## Building

`cargo build --release`

## Building to ROM file (.gba)

`./release-rom.sh`

## Running

This project uses the `mgba-qt` emulator. You can run the game with

`cargo run`

Please visit https://mgba.io/ for more information on how to install the emulator on your target platform.

## Known Issues

After a certain length of snake the game will reset without erasing the tail of the snake. In certain emulators the game will continue, however the tail will not grow after resetting. On mgba-qt I have observed the game crashing.

This will be addressed in a future update.
