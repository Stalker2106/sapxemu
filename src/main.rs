use std::{cell::RefCell, env, fs, rc::Rc};

use alu::ALU;
use bus::Bus;
use clock::Clock;
use controller::Controller;
use memory::RAM;
use pc::ProgramCounter;
use register::{RORegister, RWRegister};


mod bitset;
mod bus;
mod clock;
mod controller;
mod register;
mod memory;
mod pc;
mod alu;

// Main function to test the system
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut ramdump: Option<String> = None;

    if args.len() > 1 {
        let file_path = &args[1];
        // Check if the file exists and read its contents
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                for byte in contents.as_bytes() {
                    // Print each byte as an 8-bit binary string
                    println!("{:08b}", byte);
                }
                ramdump = Some(contents);
                println!("Successfully imported bin file: {}", file_path);
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                return;
            }
        }
    } else {
        println!("No bin file provided. Running with empty RAM.");
    }

    // Emu
    let mut bus = Bus::new();
    let mut pc = ProgramCounter::new();
    let reg_a = Rc::new(RefCell::new(RWRegister::new()));
    let reg_b: Rc<RefCell<RWRegister>> = Rc::new(RefCell::new(RWRegister::new()));
    let mut alu = ALU::new(Rc::clone(&reg_a), Rc::clone(&reg_b));
    let mar = Rc::new(RefCell::new(RORegister::new()));
    let mut ram = RAM::new(Rc::clone(&mar), ramdump);
    let mut ir = RWRegister::new();
    let reg_out = Rc::new(RefCell::new(RORegister::new()));

    let mut clock = Clock::new(1);
    let mut controller = Controller::new(
        &mut clock,
        &mut pc,
        Rc::clone(&reg_a),
        Rc::clone(&reg_b),
        &mut alu,
        Rc::clone(&mar),
        &mut ram,
        &mut bus,
        &mut ir,
        Rc::clone(&reg_out)
    );

    // Run
    controller.run();
}
