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

pub fn bitvec_to_usize(bitvec: &BitVec) -> usize {
    let mut value = 0usize;

    // Iterate over the bits in the BitVec
    for (index, bit) in bitvec.iter().enumerate() {
        // If the bit is set, add the corresponding power of 2
        if bit {
            value |= 1 << index; // Set the bit at the current index
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
