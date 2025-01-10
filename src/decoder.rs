use std::collections::HashMap;

use bit_vec::BitVec;

use crate::{config::OPCODE_SIZE, control::ControlLine, BinaryDisplay};

#[derive(PartialEq, Eq, Hash)]
pub enum OpCode {
    NOP, //No Operation
    LDA, //Load A
    ADD, //Add
    SUB, //Subtract
    MUL, //Multiply
    OUT, //Output
    HLT  //Halt
}

pub struct Decoder {
    opcodes: HashMap<OpCode, BitVec>
}

impl Decoder {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(OpCode::NOP, BitVec::from_bytes(&[0b00000000]));
        map.insert(OpCode::LDA, BitVec::from_bytes(&[0b10000000]));
        map.insert(OpCode::ADD, BitVec::from_bytes(&[0b01000000]));
        map.insert(OpCode::SUB, BitVec::from_bytes(&[0b11000000]));
        map.insert(OpCode::MUL, BitVec::from_bytes(&[0b00100000]));
        map.insert(OpCode::OUT, BitVec::from_bytes(&[0b10100000]));
        map.insert(OpCode::HLT, BitVec::from_bytes(&[0b11110000]));
        for bitvec in map.values_mut() {
            bitvec.truncate(OPCODE_SIZE);
        }
        Self {
            opcodes: map
        }
    }

    pub fn decode_instruction(&self, instruction: BitVec) -> Vec<Vec<ControlLine>> {
        // Loop over each opcode and check if it matches the instruction
        for (opcode, opcode_bits) in &self.opcodes {
            if instruction == *opcode_bits {
                match opcode {
                    OpCode::NOP => {
                        // NOP: No Operation
                        println!("NOP: No operation performed");
                        return vec![];
                    }
                    OpCode::LDA => {
                        // LDA: Load A
                        println!("LDA: Load A");
                        return vec![
                            vec![ControlLine::MI],
                            vec![ControlLine::RO, ControlLine::AI],
                        ];
                    }
                    OpCode::ADD => {
                        // ADD: Add
                        println!("ADD: Add");
                        return vec![
                            vec![ControlLine::MI],
                            vec![ControlLine::RO, ControlLine::BI],
                            vec![ControlLine::EO, ControlLine::AI],
                        ];
                    }
                    OpCode::SUB => {
                        // SUB: Subtract
                        println!("SUB: Subtract");
                        return vec![
                            vec![ControlLine::MI],
                            vec![ControlLine::RO, ControlLine::BI],
                            vec![ControlLine::SU, ControlLine::AI],
                        ];
                    }
                    OpCode::MUL => {
                        // MUL: Multiply
                        println!("MUL: Multiply");
                        return vec![];
                    }
                    OpCode::OUT => {
                        // OUT: Output
                        println!("OUT: Output");
                        return vec![vec![ControlLine::AO, ControlLine::OI]];
                    }
                    OpCode::HLT => {
                        // HLT: Halt
                        println!("HLT: Halt");
                        return vec![vec![ControlLine::HLT]];
                    }
                }
            }
        }
        println!("{}: No matching Opcode", instruction.to_bin_string());
        return vec![]
    }
}