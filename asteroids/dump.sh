#!/bin/sh
cargo build --examples --verbose
arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std target/thumbv4t-none-eabi/release/asteroids >target/dump.s
#arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std target/thumbv4t-none-eabi/release/asm_scratch >target/dump-asm_scratch.txt
