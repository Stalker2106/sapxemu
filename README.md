SAP-X Emulator

![Window](media/window.png)

A Rust-based emulator for the SAP architecture (Ben Eater's design), featuring configurable components like bus size, clock frequency, and more. It uses terminal capabilities to render a rich interface for an interactive and visually enhanced experience.
Features

    Full SAP architecture emulation
    Configurable bus size, clock frequency, memory, and CPU speed
    Step-through instruction execution
    Rich terminal-based interface for interactive visualization
    Written in Rust for performance

Getting Started

To run the emulator, simply use Cargo. You can provide a bin file as the contents of the RAM for the program instance.
Running the Emulator

cargo run <optional RAM dump.bin>

Terminal-based Interface

The emulator renders the state of the SAP architecture using advanced terminal capabilities, providing a dynamic and visually interactive experience. You’ll be able to view components like registers, the bus, ALU, and memory in a rich format as the simulation runs. The interface allows for step-through execution and easy monitoring of the system's state.

This updated README introduces the terminal rendering feature and highlights the interactive aspects of the emulator.