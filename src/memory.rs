use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bitvecutils::{bitvec_to_usize, get_bitvec_subset, inject_vec_into_bitset}, bus::Bus, clock::ClockDriven, config::{RAM_SIZE, WORD_SIZE}, control::ControlLine, link::Link, register::RORegister};

// RAM
pub struct RAM {
    control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    bus: Rc<RefCell<Bus>>,
    memory: BitVec,
    mar: Rc<RefCell<RORegister>>,
}

impl RAM {
    pub fn new(all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>, bus: Rc<RefCell<Bus>>, mar: Rc<RefCell<RORegister>>, ramdump: &Vec<u8>) -> Self {
        let mut mem = BitVec::from_elem(RAM_SIZE, false);
        if ramdump.len() > 0 {
            let mut address = 0;
            println!("address |  data");
            println!("--------|--------");
            for byte in ramdump {
                // Print each byte as an 8-bit binary string
                println!("{:08b}|{:08b}", address, byte);
                address += 1;
            }
            inject_vec_into_bitset(&mut mem, ramdump);
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
            let index = bitvec_to_usize(&self.mar.borrow().read()) * WORD_SIZE;
            let value = &get_bitvec_subset(&self.memory, index, WORD_SIZE);
            self.bus.borrow_mut().write(value);
        }
    }
}