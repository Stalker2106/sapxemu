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
    pub fn new(all_control_links: &'a HashMap<ControlLine, Rc<RefCell<Link>>>, bus: Rc<RefCell<Bus>>) -> Self {
        let mut control_eps = HashMap::new();
        control_eps.insert(ControlLine::CO, Rc::clone(&all_control_links[&ControlLine::CO]));
        control_eps.insert(ControlLine::J, Rc::clone(&all_control_links[&ControlLine::J]));
        control_eps.insert(ControlLine::CE, Rc::clone(&all_control_links[&ControlLine::CE]));
        Self {
            control_links: control_eps,
            bus,
            address: BitVec::from_elem(WORD_SIZE - OPCODE_SIZE, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.address.clone();
    }
}

impl ClockDriven for ProgramCounter {
    fn on_clock_pulse(&mut self) {
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