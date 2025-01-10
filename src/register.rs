use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bit_vec::BitVec;

use crate::{bus::{Bus, BusSelector}, clock::ClockDriven, control::ControlLine, link::Link};

// RORegister
pub struct RORegister {
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    bus: Rc<RefCell<Bus>>,
    bus_selector: BusSelector,
    regin_ctrl: ControlLine,
    value: BitVec,
}

impl RORegister {
    pub fn new(size: usize, bus: Rc<RefCell<Bus>>, bus_selector: BusSelector, regin_ctrl: ControlLine, all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>) -> Self {
        let mut control_eps = HashMap::new();
        control_eps.insert(regin_ctrl.clone(), Rc::clone(&all_control_links[&regin_ctrl]));
        Self {
            control_links: control_eps,
            bus,
            bus_selector,
            regin_ctrl,
            value: BitVec::from_elem(size, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.value.clone();
    }
}

impl ClockDriven for RORegister {
    fn on_clock_pulse(&mut self) {
        if self.control_links[&self.regin_ctrl].borrow().get_state() {
            self.value = self.bus.borrow().read_part(self.value.len(), self.bus_selector);
        }
    }
}

// RWRegister
pub struct RWRegister {
    pub control_links: HashMap<ControlLine, Rc<RefCell<Link>>>,
    regin_ctrl: ControlLine,
    regout_ctrl: ControlLine,
    bus: Rc<RefCell<Bus>>,
    bus_selector: BusSelector,
    value: BitVec,
}

impl RWRegister {
    pub fn new(size: usize, bus: Rc<RefCell<Bus>>, bus_selector: BusSelector, regin_ctrl: ControlLine, regout_ctrl: ControlLine, all_control_links: &HashMap<ControlLine, Rc<RefCell<Link>>>) -> Self {
        let mut control_eps = HashMap::new();
        control_eps.insert(regin_ctrl.clone(), Rc::clone(&all_control_links[&regin_ctrl]));
        control_eps.insert(regout_ctrl.clone(), Rc::clone(&all_control_links[&regout_ctrl]));
        Self {
            control_links: control_eps,
            bus,
            bus_selector,
            regin_ctrl,
            regout_ctrl,
            value: BitVec::from_elem(size, false)
        }
    }

    pub fn read(&self) -> BitVec {
        return self.value.clone();
    }
}

impl ClockDriven for RWRegister {
    fn on_clock_pulse(&mut self) {
        if self.control_links[&self.regin_ctrl].borrow().get_state() {
            self.value = self.bus.borrow().read_part(self.value.len(), self.bus_selector);
        }
        if self.control_links[&self.regout_ctrl].borrow().get_state() {
            self.bus.borrow_mut().write(&self.value);
        }
    }
}