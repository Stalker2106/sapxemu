use std::{cell::RefCell, rc::Rc};

use crate::{bus::{Bus, BusReader, BusWriter}, clock::ClockDriven, controller::FlaggableIC, register::RORegister};

// RAM flags
pub const RAM_RI: u8 = 0b00000001; // RAM draw from bus
pub const RAM_RO: u8 = 0b00000001; // RAM writes to bus

// RAM
pub struct RAM {
    flags: u8,
    memory: Vec<u8>,
    mar: Rc<RefCell<RORegister>>,
}

impl RAM {
    pub fn new(mar: Rc<RefCell<RORegister>>, ramdump: Option<String>) -> Self {
        let mem: Vec<u8> = if let Some(dump) = ramdump {
            let mut result = dump.as_bytes().to_vec();
            result.resize(256, 0b00000000);
            result
        } else {
            vec![0b00000000; 256]
        };
        Self {
            flags: 0,
            memory: mem,
            mar
        }
    }
}

impl BusReader for RAM {
    fn read_from_bus(&mut self, bus: &Bus) {
        self.memory[self.mar.borrow().read() as usize] = bus.read();
    }
}

impl BusWriter for RAM {
    fn write_to_bus(&self, bus: &mut Bus) {
        bus.write(self.memory[self.mar.borrow().read() as usize]);
    }
}

impl FlaggableIC for RAM {
    fn set_flag(&mut self, flag_mask: u8, state: bool) {
        if state {
            self.flags |= flag_mask;
        } else {
            self.flags &= !flag_mask;
        }
    }

    fn get_flag(&mut self, flag_mask: u8) -> bool {
        return self.flags & flag_mask != 0;
    }
}

impl ClockDriven for RAM {
    fn on_clock_pulse(&mut self, bus: &mut Bus) {
        if self.get_flag(RAM_RI) {
            self.read_from_bus(bus);
        } else if self.get_flag(RAM_RO) {
            self.write_to_bus(bus);
        }
    }
}