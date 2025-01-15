use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bus::{Bus, BusSelector}, clock::ClockDriven, control::control::ControlLine, link::Link};

// RORegister
pub struct RORegister {
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    pub name: String,
    bus: Rc<RefCell<Bus>>,
    bus_selector: BusSelector,
    regin_ctrl: ControlLine,
    pub data: BitVec,
}

impl RORegister {
    pub fn new(name: String, size: usize, bus: Rc<RefCell<Bus>>, bus_selector: BusSelector, regin_ctrl: ControlLine, control_links: HashMap<ControlLine, Rc<RefCell<Link>>>) -> Self {
        Self {
            control_links,
            name,
            bus,
            bus_selector,
            regin_ctrl,
            data: BitVec::from_elem(size, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.data.clone();
    }
}

impl ClockDriven for RORegister {
    fn on_clock_high(&mut self) {
        if self.control_links[&self.regin_ctrl].borrow().get_state() {
            self.data = self.bus.borrow().read_part(self.data.len(), self.bus_selector);
        }
    }
}

// RWRegister
pub struct RWRegister {
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    pub name: String,
    regin_ctrl: ControlLine,
    regout_ctrl: ControlLine,
    bus: Rc<RefCell<Bus>>,
    bus_selector: BusSelector,
    pub data: BitVec,
}

impl RWRegister {
    pub fn new(name: String, size: usize, bus: Rc<RefCell<Bus>>, bus_selector: BusSelector, regin_ctrl: ControlLine, regout_ctrl: ControlLine, control_links: HashMap<ControlLine, Rc<RefCell<Link>>>) -> Self {
        Self {
            control_links,
            name,
            bus,
            bus_selector,
            regin_ctrl,
            regout_ctrl,
            data: BitVec::from_elem(size, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.data.clone();
    }
}

impl ClockDriven for RWRegister {
    fn on_clock_high(&mut self) {
        if self.control_links[&self.regin_ctrl].borrow().get_state() {
            self.data = self.bus.borrow().read_part(self.data.len(), self.bus_selector);
        }
        if self.control_links[&self.regout_ctrl].borrow().get_state() {
            self.bus.borrow_mut().write(&self.data);
        }
    }
}