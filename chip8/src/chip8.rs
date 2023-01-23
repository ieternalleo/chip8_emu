use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::instruction::INSTRUCTION_SET;
use super::{Byte, Ram, Stack, Word};

use std::{collections::VecDeque, default::Default};

#[derive(Deserialize, Serialize)]
// #[serde(default)]
pub struct Chip8 {
    pub(crate) registers: [Byte; 16],
    delay_timer: Byte,
    sound_timer: Byte,
    pub(crate) index_register: Word, // Only 12 bits are used for adressing
    pub(crate) program_counter: Word,
    pub(crate) stack_pointer: Word, // Points to the top of the stack
    #[serde(with = "BigArray")]
    pub(crate) ram: Ram,
    pub(crate) stack: Stack,
    pub(crate) curr_op: Word,
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
        let _idx = 0;
        // Decode Opcode and Execute opcode
        INSTRUCTION_SET[func as usize](self);
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
        }
    }
}

pub fn new_annn(i: &mut u16, nib: u16) {
    *i = nib;
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
    fn test_jp_opcode() {
        let mut chip: Chip8 = Chip8::new();
        let program = &[0x12, 0xF0];
        chip.load_program(program);
        chip.emulate_cycle();
        assert_eq!(chip.program_counter, 0x2F0)
    }

    #[test]
    fn test_annn_opcode() {
        let mut chip: Chip8 = Chip8::new();
        let program = &[0xA2, 0xF0];
        chip.load_program(program);
        chip.emulate_cycle();
        assert_eq!(chip.program_counter, (program.len() + 0x0200) as u16);
        assert_eq!(chip.index_register, 0x02F0);
    }

    #[test]
    fn test_bnnn_opcode() {
        let mut chip: Chip8 = Chip8::new();
        let program = &[0xB2, 0xF0];
        chip.load_program(program);
        chip.emulate_cycle();

        assert_eq!(chip.program_counter, 0x02F0);
    }

    #[test]
    fn two_opcode_program() {
        let mut chip: Chip8 = Chip8::new();

        // Load a byte into Vx and then JP to PC + Vx
        let program = &[0x60, 0xF0, 0xB2, 0xF0];
        chip.initialize_ram();
        chip.load_program(program);
        chip.emulate_cycle();
        //chip.dump_to_file("two_opcode_test.txt", 8);
        assert_eq!(chip.registers[0], 0xF0);

        // Execute JP instruction
        // chip.emulate_cycle();
        // assert_eq!(chip.program_counter, 0xF0 + 0x2F0);
    }
}
