use bit_vec::BitVec;

use crate::{bitvecutils::get_bitvec_subset, config::WORD_SIZE};

#[derive(Copy, Clone, PartialEq)]
pub enum BusSelector {
    HSB,
    LSB
}

// Shared Bus
pub struct Bus {
    pub data: BitVec,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            data: BitVec::from_elem(WORD_SIZE, false)
        }
    }

    pub fn read_part(&self, count: usize, selector: BusSelector) -> BitVec {
        let position = match selector {
            BusSelector::LSB => 0, // Starting from the beginning for HSB
            BusSelector::HSB => self.data.len() - count, // Starting from the end for LSB
        };
        // Create and return the subset
        get_bitvec_subset(&self.data, position, count)
    }

    pub fn read(&self) -> BitVec {
        return self.data.clone();
    }
    pub fn write(&mut self, value: &BitVec) {
        // Get the minimum size between the current data and the new value
        let copy_size = self.data.len().min(value.len());
    
        // Copy the bits from value into self.data, respecting the fixed size
        for i in 0..copy_size {
            self.data.set(i, value[i]);
        }
    }
}
