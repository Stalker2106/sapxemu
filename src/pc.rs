use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bitvecutils::increment_bitset, bus::Bus, clock::ClockDriven, config::{OPCODE_SIZE, WORD_SIZE}, control::control::ControlLine, link::Link};

// Program Counter
pub struct ProgramCounter {
    control_links: HashMap<ControlLine, Rc<RefCell<Link>>>, 
    bus: Rc<RefCell<Bus>>,
    pub address: BitVec,
}

impl<'a> ProgramCounter {
    pub fn new(control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,  bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            control_links,
            bus,
            address: BitVec::from_elem(WORD_SIZE - OPCODE_SIZE, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.address.clone();
    }
}

impl ClockDriven for ProgramCounter {
    fn on_clock_high(&mut self) {
        if self.control_links[&ControlLine::CO].borrow().get_state() {
            self.bus.borrow_mut().write(&self.address);
        }
        if self.control_links[&ControlLine::J].borrow().get_state() {
            self.address = self.bus.borrow().read();
        }
        if self.control_links[&ControlLine::CE].borrow().get_state() {
            increment_bitset(&mut self.address);
        }
    }
}