use bit_vec::BitVec;

pub fn get_bitvec_subset(bitvec: &BitVec, position: usize, size: usize) -> BitVec {
    // Ensure the position and size are valid
    if position + size > bitvec.len() {
        return BitVec::from_elem(size, false);
    }

    // Create a new BitVec to store the subset
    let mut subset = BitVec::new();

    // Iterate over the range from position to position + size
    for i in position..position + size {
        // Get the bit at position `i` in the original BitVec and push it to the subset
        subset.push(bitvec.get(i).unwrap_or(false));
    }

    subset
}

pub fn bitvec_to_usize(bitvec: &bit_vec::BitVec) -> usize {
    let mut value = 0usize;

    // Iterate over the bits in the BitVec and ensure correct alignment (Little Endian)
    for (index, bit) in bitvec.iter().enumerate() {
        // Shift the bit by its index position (bit position).
        if bit {
            value |= 1 << index; // Set the bit at position `index` if `bit` is `true`.
        }
    }

    value
}

pub fn increment_bitset(bitvec: &mut BitVec) {
    let mut carry = true;

    // Iterate over the bits from least significant to most significant
    for mut bit in bitvec.iter_mut() {
        if carry {
            // If carry is true, flip the bit and stop if it's 0
            if *bit == false {
                *bit = true;
                carry = false; // No further carry needed
                break;
            } else {
                *bit = false; // If bit was 1, it becomes 0 and carry continues
            }
        }
    }

    // If there is still a carry, reset
    if carry {
        *bitvec = BitVec::from_elem(bitvec.len(), false);
    }
}

pub fn reverse_bits_in_byte(byte: u8) -> u8 {
    let mut reversed = 0u8;
    for i in 0..8 {
        if byte & (1 << i) != 0 {
            reversed |= 1 << (7 - i); // Reverse bit position
        }
    }
    reversed
}

pub fn convert_ramdump_to_bitvec(ramdump: &Vec<u8>) -> BitVec {
    let mut bitvec = BitVec::new();

    // Iterate through each byte in the ramdump and reverse its bits
    for byte in ramdump {
        let reversed_byte = reverse_bits_in_byte(*byte);
        
        // Now, push each bit of the reversed byte into the BitVec
        for i in 0..8 {
            bitvec.push((reversed_byte & (1 << (7 - i))) != 0);
        }
    }

    bitvec
}

pub trait BinaryDisplay {
    fn to_bin_string(&self) -> String;
}


impl BinaryDisplay for BitVec {
    fn to_bin_string(&self) -> String {
        // Collect all bits into a string representation in correct order
        self.iter()
            .rev() // Reverse the order of bits before mapping
            .map(|bit| if bit { '1' } else { '0' })
            .collect()
    }
}
