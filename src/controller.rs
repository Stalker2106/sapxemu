use std::{cell::RefCell, process::exit, rc::Rc, sync::mpsc};

use crate::{alu::ALU, bus::{Bus, BusReader, BusWriter}, clock::Clock, memory::{RORegister, RWRegister, RAM}, ProgramCounter};

// Decoder

fn decode_instruction(instruction: u8) -> Vec<Vec<&'static str>> {
    match instruction {
        0b0000 => {
            // NOP: No Operation
            println!("NOP: No operation performed.");
            return vec![];
        }
        0b0001 => {
            // LDA: Load A
            return vec![
                vec!["CO","MI"],
                vec!["RO","AI"]
            ];
        }
        0b0010 => {
            // ADD: Add
            return vec![
                vec!["CO","MI"],
                vec!["RO","BI"],
                vec!["EO","AI"]
            ];
        }
        0b0011 => {
            // SUB: Subtract
            return vec![
                vec!["CO","MI"],
                vec!["RO","BI"],
                vec!["SU","AI"]
            ];
        }
        0b0100 => {
            // MUL: Multiply
            return vec![];
        }
        0b0101 => {
            // OUT: Output
            return vec![vec!["AO","OI"]];
        }
        0b0110 => {
            // HLT: Halt
            return vec![vec!["HLT"]];
        }
        _ => {
            println!("Unknown instruction: {:04b}", instruction);
            return vec![];
        }
    }
}

// Controller
pub struct Controller<'a> {
    fetch_microcode: Vec<Vec<&'a str>>,
    instruction_microcode: Vec<Vec<&'a str>>,
    microcode_step: u8,
    // Links
    clock: &'a mut Clock,
    pc: &'a mut ProgramCounter,
    reg_a: Rc<RefCell<RWRegister>>,
    reg_b: Rc<RefCell<RWRegister>>,
    alu: &'a mut ALU,
    mar: Rc<RefCell<RORegister>>,
    ram: &'a mut RAM,
    bus: &'a mut Bus,
    ir: &'a mut RWRegister,
    reg_out: Rc<RefCell<RORegister>>
}

impl<'a> Controller<'a> {
    pub fn new(
        clock: &'a mut Clock,
        pc: &'a mut ProgramCounter,
        reg_a: Rc<RefCell<RWRegister>>,
        reg_b: Rc<RefCell<RWRegister>>,
        alu: &'a mut ALU,
        mar: Rc<RefCell<RORegister>>,
        ram: &'a mut RAM,
        bus: &'a mut Bus,
        ir: &'a mut RWRegister,
        reg_out: Rc<RefCell<RORegister>>,
    ) -> Self {
        Self {
            fetch_microcode: vec![
                vec!["CO", "MI"],
                vec!["RO", "II"],
                vec!["CE"],
            ],
            instruction_microcode: Vec::new(),
            microcode_step: 0,
            // Links
            clock,
            pc,
            reg_a,
            reg_b,
            alu,
            mar,
            ram,
            bus,
            ir,
            reg_out
        }
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.clock.start(tx);
        loop {
            match rx.recv() {
                Ok(_) => {
                    self.on_clock_tick();
                }
                Err(_) => {
                    println!("Clock thread has stopped. Exiting main loop.");
                    break;
                }
            }
        }
    }


    pub fn on_clock_tick(&mut self) {
        if self.microcode_step < self.fetch_microcode.len() as u8 {
            let step_signals = self.fetch_microcode[self.microcode_step as usize].clone();
            // Raise all step signals
            for signal in step_signals {
                self.control(&signal);
            }
            // Increment microcode step
            self.microcode_step += 1;
            // Decode when fetch is over
            if self.microcode_step == self.fetch_microcode.len() as u8 {
                self.instruction_microcode = decode_instruction(self.ir.read());
            }
        } else {
            let rstep = self.microcode_step as usize - self.fetch_microcode.len();
            // Execute instruction microcode
            if rstep < self.instruction_microcode.len() {
                let step_signals = self.instruction_microcode[rstep].clone();
                // Raise all step signals
                for signal in step_signals {
                    self.control(&signal);
                }
            }
            // Increment microcode step
            self.microcode_step += 1;
            // Handle reaching end of cycle
            if self.microcode_step as usize >= (self.fetch_microcode.len() + self.instruction_microcode.len()) {
                self.microcode_step = 0;
            }
        }
    }

    fn control(&mut self, signal: &str) {
        match signal {
            "HLT" => {
                // Halt the computer
                exit(0);
            }
            "MI" => {
                // Memory address register in
                self.mar.borrow_mut().read_from_bus(self.bus);
            }
            "RI" => {
                // RAM in
                self.ram.read_from_bus(self.bus);
            }
            "RO" => {
                // RAM out
                self.ram.write_to_bus(self.bus);
            }
            "II" => {
                // Instruction register in
                self.ir.read_from_bus(self.bus);
            }
            "IO" => {
                // Instruction register out (not needed with 256B RAM mod)
                self.ir.write_to_bus(self.bus);
            }
            "AI" => {
                // Register A in
                self.reg_a.borrow_mut().read_from_bus(self.bus);
            }
            "AO" => {
                // Register A out
                self.reg_a.borrow().write_to_bus(self.bus);
            }
            "EO" => {
                // ALU sum out
                self.alu.write_to_bus(self.bus);
            }
            "SU" => {
                // ALU Subtract
                //NOTE: IMPLEM SUB
                self.alu.write_to_bus(self.bus);
            }
            "BI" => {
                // Register B in
                self.reg_b.borrow_mut().read_from_bus(self.bus);
            }
            "BO" => {
                // Register B in
                self.reg_b.borrow().write_to_bus(self.bus);
            }
            "OI" => {
                // Output in
                self.reg_out.borrow_mut().read_from_bus(self.bus);
            }
            "CE" => {
                // Counter enable
                self.pc.increment();
            }
            "CO" => {
                // Counter out
                self.pc.write_to_bus(self.bus);
            }
            "J" => {
                // Jump (Counter in)
                self.pc.read_from_bus(self.bus);
            }
            "FI" => {
                // Flags register in
            }
            _ => {
                println!("Unknown control {}", signal);
            }
        }
    }

}