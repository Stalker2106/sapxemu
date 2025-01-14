use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bitvecutils::{bitvec_to_usize, get_bitvec_subset}, bus::Bus, clock::ClockDriven, config::{RAM_SIZE, WORD_SIZE}, control::control::ControlLine, link::Link, memory::register::RORegister};

// RAM
pub struct RAM {
    control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    bus: Rc<RefCell<Bus>>,
    pub memory: Vec<BitVec>,
    pub mar: Rc<RefCell<RORegister>>,
}

impl RAM {
    pub fn new(all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, bus: Rc<RefCell<Bus>>, mar: Rc<RefCell<RORegister>>, ramdump: &BitVec) -> Self {
        let mut mem = Vec::new();
        for i in 0..RAM_SIZE {
            mem.push(get_bitvec_subset(ramdump, i * WORD_SIZE, WORD_SIZE));
        }
        let mut control_eps = HashMap::new();
        control_eps.insert(ControlLine::RI, Rc::clone(&all_control_links[&ControlLine::RI]));
        control_eps.insert(ControlLine::RO, Rc::clone(&all_control_links[&ControlLine::RO]));
        Self {
            control_links: control_eps,
            bus,
            memory: mem,
            mar
        }
    }
}

impl ClockDriven for RAM {
    fn on_clock_pulse(&mut self) {
        if self.control_links[&ControlLine::RI].borrow().get_state() {
            // do something
        }
        if self.control_links[&ControlLine::RO].borrow().get_state() {
            let index = bitvec_to_usize(&self.mar.borrow().read());
            self.bus.borrow_mut().write(&self.memory[index]);
        }
    }
}