use crate::{bus::{Bus, BusReader, BusWriter}, clock::ClockDriven, controller::FlaggableIC};

// PC flags
pub const PC_OUT: u8 = 0b00000001; // draws from bus
pub const PC_IN: u8 = 0b00000010; // writes to bus
pub const PC_INCR: u8 = 0b00000011; // increments

// Program Counter
pub struct ProgramCounter {
    flags: u8,
    address: u8,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self {
            flags: 0,
            address: 0
        }
    }

    pub fn spy(&self) -> u8 {
        return self.address;
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

impl FlaggableIC for ProgramCounter {
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

impl ClockDriven for ProgramCounter {
    fn on_clock_pulse(&mut self, bus: &mut Bus) {
        if self.get_flag(PC_IN) {
            self.read_from_bus(bus);
        } else if self.get_flag(PC_OUT) {
            self.write_to_bus(bus);
        } else if self.get_flag(PC_INCR) {
            self.address += 1;
        }
    }
}