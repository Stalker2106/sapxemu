use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, register::RWRegister};

// ALU signals
pub enum ALUSignal {
    EO, // Sum out to bus
    SU, // Sub out to bus
}

pub struct ALU {
    reg_a: Rc<RefCell<RWRegister>>,
    reg_b: Rc<RefCell<RWRegister>>,
}

impl ALU {
    pub fn new(reg_a: Rc<RefCell<RWRegister>>, reg_b: Rc<RefCell<RWRegister>>) -> Self {
        Self {
            reg_a,
            reg_b
        }
    }

    pub fn signal(&self, signal: ALUSignal, bus: &mut Bus) {
        match signal {
            ALUSignal::EO => {
                bus.write(self.reg_a.borrow().read() + self.reg_b.borrow().read());
            },
            ALUSignal::SU => {
                bus.write(self.reg_a.borrow().read() - self.reg_b.borrow().read());
            }
        }
    }
}

