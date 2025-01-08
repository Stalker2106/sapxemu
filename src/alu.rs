use std::{cell::RefCell, rc::Rc};

use crate::{bus::{Bus, BusWriter}, memory::RWRegister};

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
}

impl BusWriter for ALU {
    fn write_to_bus(&self, bus: &mut Bus) {
    }
}