use std::{cell::RefCell, collections::HashMap, env, fs::File, io::Read, process, rc::Rc, sync::mpsc};

use alu::ALU;
use bit_vec::BitVec;
use bus::{Bus, BusSelector};
use clock::{Clock, ClockDriven};
use config::{OPCODE_SIZE, WORD_SIZE};
use control::control::ControlLine;
use control::controller::Controller;
use events::keyboard::handle_keyboard;
use link::Link;
use memory::memory::RAM;
use pc::ProgramCounter;
use memory::register::{RORegister, RWRegister};
use display::renderer::Renderer;
use control::sequencer::Sequencer;

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

pub trait BinaryDisplay {
    fn to_bin_string(&self) -> String;
}


impl BinaryDisplay for BitVec {
    fn to_bin_string(&self) -> String {
        // Collect all bits into a string representation in correct order
        self.iter()
            .rev() // Reverse the order of bits before mapping
            .map(|bit| if bit { '1' } else { '0' })
            .collect()
    }
}

fn print_state(control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, bus: &Rc<RefCell<Bus>>, pc: &ProgramCounter, mar: &Rc<RefCell<RORegister>>, ir: &Rc<RefCell<RWRegister>>, reg_a: &Rc<RefCell<RWRegister>>, reg_b: &Rc<RefCell<RWRegister>>, reg_out: &RORegister) {
    print!("Controls:");
    for (control, link) in control_links {
        if !link.borrow().get_state() { continue; }
        print!(" {}:{}", control, link.borrow().get_state());
    }
    print!("\n");
    println!("BUS: {} | PC: {} | MAR: {} | IR: {} | A: {} | B: {} | OUT: {}", bus.borrow().read().to_bin_string(), pc.read().to_bin_string(), mar.borrow().read().to_bin_string(), ir.borrow().read().to_bin_string(), reg_a.borrow().read().to_bin_string(), reg_b.borrow().read().to_bin_string(), reg_out.read().to_bin_string());
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut ramdump: Vec<u8> = Vec::new();

    if args.len() > 1 {
        let binfile = &args[1];
        match File::open(binfile) {
            Ok(mut file) => {
                match file.read_to_end(&mut ramdump) {
                    Ok(_) => println!("Loaded bin file {} into RAM", binfile),
                    Err(_) => eprintln!("Error loading bin file {} into RAM", binfile),
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

    let renderer = Rc::new(RefCell::new(Renderer::new()));

    // Control Links
    let mut control_links: HashMap<ControlLine, Rc<RefCell<Link>>> = HashMap::new();
    control_links.insert(ControlLine::HLT, Rc::new(RefCell::new(Link::new(ControlLine::HLT))));
    control_links.insert(ControlLine::MI, Rc::new(RefCell::new(Link::new(ControlLine::MI))));
    control_links.insert(ControlLine::RI, Rc::new(RefCell::new(Link::new(ControlLine::RI))));
    control_links.insert(ControlLine::RO, Rc::new(RefCell::new(Link::new(ControlLine::RO))));
    control_links.insert(ControlLine::II, Rc::new(RefCell::new(Link::new(ControlLine::II))));
    control_links.insert(ControlLine::IO, Rc::new(RefCell::new(Link::new(ControlLine::IO))));
    control_links.insert(ControlLine::AI, Rc::new(RefCell::new(Link::new(ControlLine::AI))));
    control_links.insert(ControlLine::AO, Rc::new(RefCell::new(Link::new(ControlLine::AO))));
    control_links.insert(ControlLine::EO, Rc::new(RefCell::new(Link::new(ControlLine::EO))));
    control_links.insert(ControlLine::SU, Rc::new(RefCell::new(Link::new(ControlLine::SU))));
    control_links.insert(ControlLine::BI, Rc::new(RefCell::new(Link::new(ControlLine::BI))));
    control_links.insert(ControlLine::BO, Rc::new(RefCell::new(Link::new(ControlLine::BO))));
    control_links.insert(ControlLine::OI, Rc::new(RefCell::new(Link::new(ControlLine::OI))));
    control_links.insert(ControlLine::CE, Rc::new(RefCell::new(Link::new(ControlLine::CE))));
    control_links.insert(ControlLine::CO, Rc::new(RefCell::new(Link::new(ControlLine::CO))));
    control_links.insert(ControlLine::J, Rc::new(RefCell::new(Link::new(ControlLine::J))));
    control_links.insert(ControlLine::FI, Rc::new(RefCell::new(Link::new(ControlLine::FI))));

    // Emu
    let bus = Rc::new(RefCell::new(Bus::new()));

    let mut pc = ProgramCounter::new(&control_links, Rc::clone(&bus));

    let reg_a = Rc::new(RefCell::new(RWRegister::new(
        "A Register".to_string(),
        WORD_SIZE,
        Rc::clone(&bus),
        BusSelector::LSB,
        ControlLine::AI,
        ControlLine::AO,
        &control_links
    )));

    let reg_b = Rc::new(RefCell::new(RWRegister::new(
        "B Register".to_string(),
        WORD_SIZE,
        Rc::clone(&bus),
        BusSelector::LSB,
        ControlLine::BI,
        ControlLine::BO,
        &control_links
    )));

    let alu = Rc::new(RefCell::new(ALU::new(
        &control_links,
        Rc::clone(&bus),
        Rc::clone(&reg_a),
        Rc::clone(&reg_b)))
    );
    control_links[&ControlLine::EO].borrow_mut().add_callback(Box::new({
        let calu = Rc::clone(&alu);
        move || calu.borrow().add()
    }));
    control_links[&ControlLine::SU].borrow_mut().add_callback(Box::new({
        let calu = Rc::clone(&alu);
        move || calu.borrow().sub()
    }));

    let mar = Rc::new(RefCell::new(RORegister::new(
        "Mem. Address Reg.".to_string(),
        WORD_SIZE - OPCODE_SIZE,
        Rc::clone(&bus),
        BusSelector::LSB,
        ControlLine::MI,
        &control_links
    )));

    let mut ram = RAM::new(&control_links, Rc::clone(&bus), Rc::clone(&mar), &BitVec::from_bytes(&ramdump));

    let ir = Rc::new(RefCell::new(RWRegister::new(
        "Instruction Reg.".to_string(),
        OPCODE_SIZE,
        Rc::clone(&bus),
        BusSelector::HSB,
        ControlLine::II,
        ControlLine::IO,
        &control_links
    )));

    let mut reg_out = RORegister::new(
        "Output Register".to_string(),
        WORD_SIZE,
        Rc::clone(&bus),
        BusSelector::LSB,
        ControlLine::OI,
        &control_links
    );

    let mut clock = Clock::new(1);
    let sequencer = Rc::new(RefCell::new(Sequencer::new()));
    let mut controller = Controller::new(
        Rc::clone(&sequencer),
        &control_links
    );
    control_links[&ControlLine::HLT].borrow_mut().add_callback(Box::new({
        let renderer = Rc::clone(&renderer); // Clone Rc to use it inside the closure
        move || {
            renderer.borrow_mut().stop();
            std::process::exit(1);
        }
    }));


    // Run
    let (tx, rx) = mpsc::channel();
    clock.start(tx);
    loop {
        handle_keyboard(&control_links).unwrap();
        match rx.recv() {
            Ok(_) => {
                controller.on_clock_pulse();
                pc.on_clock_pulse();
                mar.borrow_mut().on_clock_pulse();
                ram.on_clock_pulse();
                ir.borrow_mut().on_clock_pulse();
                reg_a.borrow_mut().on_clock_pulse();
                reg_b.borrow_mut().on_clock_pulse();
                reg_out.on_clock_pulse();
                // Increment microcode step
                renderer.borrow_mut().draw(&control_links, &controller, &bus, &pc, &alu, &mar, &ram, &ir, &reg_a, &reg_b, &reg_out);
                sequencer.borrow_mut().increment_step(&ir);
            }
            Err(_) => {
                println!("Clock thread has stopped. Exiting main loop.");
                break;
            }
        }
    }
}
