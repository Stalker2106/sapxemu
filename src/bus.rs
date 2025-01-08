// Shared Bus
pub struct Bus {
    data: u8,
}

impl Bus {
    pub fn new() -> Self {
        Self { data: 0 }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, value: u8) {
        self.data = value;
    }
}

// Traits for specialized IC behavior
pub trait BusReader {
    fn read_from_bus(&mut self, bus: &Bus);
}

pub trait BusWriter {
    fn write_to_bus(&self, bus: &mut Bus);
}