use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bus::Bus, control::control::ControlLine, link::Link, memory::register::RWRegister};

fn sum_bitvecs(a: &BitVec, b: &BitVec) -> BitVec {
    let mut result = BitVec::new();
    let mut carry = false;  // This will hold the carry bit during the addition
    
    let max_len = std::cmp::max(a.len(), b.len());  // Find the maximum length of the two BitVecs
    for i in 0..max_len {
        let bit_a = a.get(i).unwrap_or(false);  // Get the bit at position i (defaulting to false if out of bounds)
        let bit_b = b.get(i).unwrap_or(false);  // Get the bit at position i (defaulting to false if out of bounds)

        // Perform the addition with carry
        let sum = bit_a as u8 + bit_b as u8 + carry as u8;
        result.push(sum & 1 == 1);  // Store the least significant bit of the sum
        carry = sum > 1;  // If sum > 1, there is a carry
    }

    if carry {
        result.push(true);  // If there's a carry left after the last bit, push a '1' to the result
    }

    result
}

pub struct ALU {
    control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    bus: Rc<RefCell<Bus>>,
    reg_a: Rc<RefCell<RWRegister>>,
    reg_b: Rc<RefCell<RWRegister>>,
}

impl ALU {
    pub fn new(control_links: HashMap<ControlLine, Rc<RefCell<Link>>>, bus: Rc<RefCell<Bus>>, reg_a: Rc<RefCell<RWRegister>>, reg_b: Rc<RefCell<RWRegister>>) -> Self {
        Self {
            control_links,
            bus,
            reg_a,
            reg_b
        }
    }

    pub fn add(&self) {
        self.bus.borrow_mut().write(&sum_bitvecs(&self.reg_a.borrow().read(), &self.reg_b.borrow().read()));
    }

    pub fn sub(&self) {
        self.bus.borrow_mut().write(&sum_bitvecs(&self.reg_a.borrow().read(), &self.reg_b.borrow().read()));
    }
}

