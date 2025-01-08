use crate::bus::{Bus, BusReader, BusWriter};


// Program Counter
pub struct ProgramCounter {
    address: u8,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { address: 0 }
    }

    pub fn increment(&mut self) {
        self.address += 1;
    }
}

impl BusReader for ProgramCounter {
    fn read_from_bus(&mut self, bus: &Bus) {
        self.address = bus.read();
    }
}

impl BusWriter for ProgramCounter {
    fn write_to_bus(&self, bus: &mut Bus) {
        bus.write(self.address);
    }
}
