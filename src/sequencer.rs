use std::{cell::RefCell, rc::Rc};

use crate::{control::ControlLine, decoder::Decoder, register::RWRegister};


// Sequencer
pub struct Sequencer {
    fetch_microcode: Vec<Vec<ControlLine>>,
    instruction_microcode: Vec<Vec<ControlLine>>,
    microcode_step: u8,
    decoder: Decoder
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            fetch_microcode: vec![
                vec![ControlLine::CO, ControlLine::MI],
                vec![ControlLine::RO, ControlLine::II],
                vec![ControlLine::CE],
            ],
            instruction_microcode: Vec::new(),
            microcode_step: 0,
            decoder: Decoder::new()
        }
    }

    pub fn get_current_step_controls(&self) -> Vec<ControlLine> {
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

    pub fn get_prev_step_controls(&self) -> Vec<ControlLine> {
        if self.microcode_step > 0 {
            let prev_step = self.microcode_step - 1;
            if prev_step < self.fetch_microcode.len() as u8 {
                return self.fetch_microcode[prev_step as usize].clone();
            } else {
                let rstep = prev_step as usize - self.fetch_microcode.len();
                if rstep < self.instruction_microcode.len() {
                    return self.instruction_microcode[rstep].clone();
                }
            }
        } else {
            if self.instruction_microcode.len() > 0 {
                return self.instruction_microcode[self.instruction_microcode.len()-1].clone();
            } else {
                return self.fetch_microcode[self.fetch_microcode.len()-1].clone();
            }
        }
        return vec![]
    }

    pub fn increment_step(&mut self, ir: &Rc<RefCell<RWRegister>>) {
        self.microcode_step += 1;
        // Handle instruction decoding when fetch is over
        if self.microcode_step == self.fetch_microcode.len() as u8 {
            self.instruction_microcode = self.decoder.decode_instruction(ir.borrow().read());
        }
        // Handle cycle end
        if self.microcode_step as usize >= (self.fetch_microcode.len() + self.instruction_microcode.len()) {
            self.microcode_step = 0;
            println!("=====");
        }
    }
}
