use std::{cell::RefCell, env, fs::File, io::Read, rc::Rc, sync::mpsc};

use clock::Clock;
use computer::Computer;
use config::CLOCK_FREQUENCY;
use display::renderer::Renderer;
use events::keyboard::handle_keyboard;

mod display;
mod memory;
mod bitvecutils;
mod config;
mod control;
mod bus;
mod clock;
mod pc;
mod alu;
mod link;
mod events;
mod computer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ramdump: Vec<u8> = Vec::new();

    if args.len() > 1 {
        let binfile = &args[1];
        match File::open(binfile) {
            Ok(mut file) => {
                if file.read_to_end(&mut ramdump).is_err() {
                    eprintln!("Error loading bin file {} into RAM", binfile);
                } else {
                    println!("Loaded bin file {} into RAM", binfile);
                }
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", binfile, e);
                return;
            }
        }
    } else {
        println!("No bin file provided. Running with empty RAM.");
    }

    let clock = Rc::new(RefCell::new(Clock::new(CLOCK_FREQUENCY)));
    let mut computer = Computer::new(ramdump);

    let renderer = Rc::new(RefCell::new(Renderer::new()));

    let (tx, rx) = mpsc::channel();
    clock.borrow_mut().start(tx);

    loop {
        handle_keyboard(&renderer, &computer.control_links).unwrap();
        match rx.recv() {
            Ok(clock_state) => {
                computer.on_clock_high(clock_state);
                renderer.borrow_mut().draw(&clock, &computer);
            }
            Err(_) => {
                // Clock stopped
                break;
            }
        }
    }
}