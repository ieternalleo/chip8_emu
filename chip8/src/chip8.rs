use lazy_static::*;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::instruction::initialize_instruction_set;
use super::instruction::{Instruction, InstructionSet, INSTRUCTION_SET};
use super::{Byte, Ram, Stack, Word};

use std::{collections::HashMap, collections::VecDeque, default::Default, hash::Hash};

#[derive(Deserialize, Serialize)]
// #[serde(defaulst)]
pub struct Chip8 {
    pub(crate) registers: [Byte; 16],
    pub(crate) delay_timer: Byte,
    pub(crate) sound_timer: Byte,
    pub(crate) index_register: Word, // Only 12 bits are used for adressing
    pub(crate) program_counter: Word,
    pub(crate) stack_pointer: Word, // Points to the top of the stack
    #[serde(with = "BigArray")]
    pub(crate) ram: Ram,
    pub(crate) stack: Stack,
    pub(crate) curr_op: Word,
    instructions: InstructionSet,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            program_counter: 0x0200,
            ..Default::default()
        }
    }
    // pub fn initialize(&mut self) {
    //     self.
    // }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode from MEMORY[PC] ( |OpCode| = 1 WORD )
        self.curr_op = self.read_word(self.program_counter as usize);
        self.program_counter += 2;

        let func = (self.curr_op & 0xF000) >> 12;
        let idx = 0;
        // Decode Opcode
        match func {
            0x0 => {
                let operation = match self.curr_op & 0x000F {
                    0x0 => INSTRUCTION_SET.get("00E0").unwrap(),
                    0xE => INSTRUCTION_SET.get("00EE").unwrap(),
                    _ => unimplemented!(),
                };
                operation.execute()(self)
            }
            0x1 => INSTRUCTION_SET.get("1NNN").unwrap().execute()(self),
            0x2 => INSTRUCTION_SET.get("2NNN").unwrap().execute()(self),
            0x3 => INSTRUCTION_SET.get("3XKK").unwrap().execute()(self),
            0x4 => INSTRUCTION_SET.get("4XKK").unwrap().execute()(self),
            0x5 => INSTRUCTION_SET.get("5XY0").unwrap().execute()(self),
            0x6 => INSTRUCTION_SET.get("6XKK").unwrap().execute()(self),
            0x7 => INSTRUCTION_SET.get("7XKK").unwrap().execute()(self),
            0xA => INSTRUCTION_SET.get("ANNN").unwrap().execute()(self),
            _ => unimplemented!(),
        }

        // Execute opcode

        // Update Timers
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            ram: [0u8; 4096],
            stack: VecDeque::with_capacity(4096),
            registers: [0u8; 16],
            delay_timer: 0,
            sound_timer: 0,
            index_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            curr_op: 0x0000,
            instructions: initialize_instruction_set(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;
    #[test]
    fn test_placeholder() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_annn_opcode() {
        let mut chip: Chip8 = Chip8::new();
        chip.load_program(&[0xA2, 0xF0]);
        chip.emulate_cycle();
        assert_eq!(chip.program_counter, 0x0200);
        assert_eq!(chip.index_register, 0x02F0);
    }
    
}
