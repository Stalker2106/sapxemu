use crate::{bus::{Bus, BusReader, BusWriter}, clock::ClockDriven, controller::FlaggableIC};


// Registers flags
pub const REG_IN: u8 = 0b00000001; // draws from bus
pub const REG_OUT: u8 = 0b00000010; // writes to bus

// RORegister
pub struct RORegister {
    flags: u8,
    value: u8,
}

impl RORegister {
    pub fn new() -> Self {
        Self {
            flags: 0,
            value: 0
        }
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

impl FlaggableIC for RORegister {
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

impl ClockDriven for RORegister {
    fn on_clock_pulse(&mut self, bus: &mut Bus) {
        if self.get_flag(REG_IN) {
            self.read_from_bus(bus);
        }
    }
}

// RWRegister
pub struct RWRegister {
    flags: u8,
    value: u8,
}

impl RWRegister {
    pub fn new() -> Self {
        Self {
            flags: 0,
            value: 0
        }
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

impl FlaggableIC for RWRegister {
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

impl ClockDriven for RWRegister {
    fn on_clock_pulse(&mut self, bus: &mut Bus) {
        if self.get_flag(REG_IN) {
            self.read_from_bus(bus);
        } else if self.get_flag(REG_OUT) {
            self.write_to_bus(bus);
        }
    }
}