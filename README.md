# sapxemu

A Rust-based emulator for the SAP architecture (Ben Eater's design), featuring configurable components like bus size, clock frequency, and more.

## Features
- Full SAP architecture emulation
- Configurable bus size, clock frequency, memory, and CPU speed
- Step-through instruction execution
- Written in Rust for performance

## Getting Started

To run the emulator, simply use cargo. You can provide a bin file as the contents of the RAM for the program instance.

    cargo run <optional RAM dump.bin>