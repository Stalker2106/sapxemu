use std::{cell::RefCell, rc::Rc};

use crate::bus::{Bus, BusReader, BusWriter};

// RAM
pub struct RAM {
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

// RORegister
pub struct RORegister {
    value: u8,
}

impl RORegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn read(&self) -> u8 {
        return self.value;
    }
}

impl BusReader for RORegister {
    fn read_from_bus(&mut self, bus: &Bus) {
        self.value = bus.read();
    }
}

// RWRegister
pub struct RWRegister {
    value: u8,
}

impl RWRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn read(&self) -> u8 {
        return self.value;
    }
}

impl BusReader for RWRegister {
    fn read_from_bus(&mut self, bus: &Bus) {
        self.value = bus.read();
    }
}

impl BusWriter for RWRegister {
    fn write_to_bus(&self, bus: &mut Bus) {
        bus.write(self.value);
    }
}
