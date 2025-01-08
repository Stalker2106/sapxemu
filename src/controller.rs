use std::{cell::RefCell, process::exit, rc::Rc, sync::mpsc};

use crate::{alu::{ALUSignal, ALU}, bus::Bus, clock::{Clock, ClockDriven}, memory::{RAM, RAM_RI, RAM_RO}, pc::{PC_IN, PC_INCR, PC_OUT}, register::{RORegister, RWRegister, REG_IN, REG_OUT}, ProgramCounter};

// Decoder

fn decode_instruction(instruction: u8) -> Vec<Vec<String>> {
    match instruction {
        0b0000 => {
            // NOP: No Operation
            println!("NOP: No operation performed.");
            return vec![];
        }
        0b0001 => {
            // LDA: Load A
            return vec![
                vec!["CO".to_string(), "MI".to_string()],
                vec!["RO".to_string(), "AI".to_string()]
            ];
        }
        0b0010 => {
            // ADD: Add
            return vec![
                vec!["CO".to_string(), "MI".to_string()],
                vec!["RO".to_string(), "BI".to_string()],
                vec!["EO".to_string(), "AI".to_string()]
            ];
        }
        0b0011 => {
            // SUB: Subtract
            return vec![
                vec!["CO".to_string(), "MI".to_string()],
                vec!["RO".to_string(), "BI".to_string()],
                vec!["SU".to_string(), "AI".to_string()]
            ];
        }
        0b0100 => {
            // MUL: Multiply
            return vec![];
        }
        0b0101 => {
            // OUT: Output
            return vec![vec!["AO".to_string(), "OI".to_string()]];
        }
        0b0110 => {
            // HLT: Halt
            return vec![vec!["HLT".to_string()]];
        }
        _ => {
            println!("Unknown instruction: {:04b}", instruction);
            return vec![];
        }
    }
}

// Sequencer
pub struct Sequencer {
    fetch_microcode: Vec<Vec<String>>,
    instruction_microcode: Vec<Vec<String>>,
    microcode_step: u8
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            fetch_microcode: vec![
                vec!["CO".to_string(), "MI".to_string()],
                vec!["RO".to_string(), "II".to_string()],
                vec!["CE".to_string()],
            ],
            instruction_microcode: Vec::new(),
            microcode_step: 0,
        }
    }

    pub fn get_current_step_controls(&self) -> Vec<String> {
        if self.microcode_step < self.fetch_microcode.len() as u8 {
            return self.fetch_microcode[self.microcode_step as usize].clone();
        } else {
            let rstep = self.microcode_step as usize - self.fetch_microcode.len();
            if rstep < self.instruction_microcode.len() {
                return self.instruction_microcode[rstep].clone();
            }
        }
        return vec![]
    }

    pub fn increment_step(&mut self, ir: &mut RWRegister) {
        self.microcode_step += 1;
        // Handle instruction decoding when fetch is over
        if self.microcode_step == self.fetch_microcode.len() as u8 {
            self.instruction_microcode = decode_instruction(ir.read());
        }
        // Handle cycle end
        if self.microcode_step as usize >= (self.fetch_microcode.len() + self.instruction_microcode.len()) {
            self.microcode_step = 0;
        }
    }
}

// Controller
pub struct Controller<'a> {
    clock: &'a mut Clock,
    sequencer: Sequencer,
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
            clock,
            sequencer: Sequencer::new(),
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
                    self.print_state();
                    self.on_clock_pulse();
                    self.pc.on_clock_pulse(self.bus);
                    self.mar.borrow_mut().on_clock_pulse(self.bus);
                    self.ir.on_clock_pulse(self.bus);
                    self.reg_a.borrow_mut().on_clock_pulse(self.bus);
                    self.reg_b.borrow_mut().on_clock_pulse(self.bus);
                    self.reg_out.borrow_mut().on_clock_pulse(self.bus);
                }
                Err(_) => {
                    println!("Clock thread has stopped. Exiting main loop.");
                    break;
                }
            }
        }
    }

    pub fn print_state(&self) {
        println!("BUS: {:08b}", self.bus.read());
        println!("PC: {:08b}", self.pc.spy());
    }

    pub fn set_step_controls(&mut self, microcode_step: Vec<String>, state: bool) {
        for signal in microcode_step {
            self.set_control(&signal, state);
        }
    }

    pub fn on_clock_pulse(&mut self) {
        // Drive Prev step controls low
        let prev_step_controls = self.sequencer.get_current_step_controls();
        self.set_step_controls(prev_step_controls, false);
        // Run current step
        let step_controls = self.sequencer.get_current_step_controls();
        self.set_step_controls(step_controls, true);
        // Increment microcode step
        self.sequencer.increment_step(self.ir);
    }

    fn set_control(&mut self, signal: &str, state: bool) {
        match signal {
            "HLT" => {
                // Halt the computer
                exit(0);
            }
            "MI" => {
                // Memory address register in
                self.mar.borrow_mut().set_flag(REG_IN, state);
            }
            "RI" => {
                // RAM in
                self.ram.set_flag(RAM_RI, state);
            }
            "RO" => {
                // RAM out
                self.ram.set_flag(RAM_RO, state);
            }
            "II" => {
                // Instruction register in
                self.ir.set_flag(REG_IN, state);
            }
            "IO" => {
                // Instruction register out (not needed with 256B RAM mod)
                self.ir.set_flag(REG_OUT, state);
            }
            "AI" => {
                // Register A in
                self.reg_a.borrow_mut().set_flag(REG_IN, state);
            }
            "AO" => {
                // Register A out
                self.reg_a.borrow_mut().set_flag(REG_OUT, state);
            }
            "EO" => {
                // ALU sum out
                if state { self.alu.signal(ALUSignal::EO, self.bus) };
            }
            "SU" => {
                // ALU Subtract
                if state { self.alu.signal(ALUSignal::SU, self.bus) };
            }
            "BI" => {
                // Register B in
                self.reg_b.borrow_mut().set_flag(REG_IN, state);
            }
            "BO" => {
                // Register B out
                self.reg_b.borrow_mut().set_flag(REG_OUT, state);
            }
            "OI" => {
                // Output in
                self.reg_out.borrow_mut().set_flag(REG_IN, state);
            }
            "CE" => {
                // Counter enable
                self.pc.set_flag(PC_INCR, state);
            }
            "CO" => {
                // Counter out
                self.pc.set_flag(PC_OUT, state);
            }
            "J" => {
                // Jump (Counter in)
                self.pc.set_flag(PC_IN, state);
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

pub trait FlaggableIC {
    fn set_flag(&mut self, flag_mask: u8, state: bool);
    fn get_flag(&mut self, flag_mask: u8) -> bool;
}