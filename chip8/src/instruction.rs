use crate::chip8::Chip8;
use lazy_static::*;
//use rand::prelude::*;

use std::default;
type OpCode = u16;
type Instr = fn(&mut Chip8) -> ();
pub(crate) type InstructionSet = Vec<Instruction>;

#[derive(serde::Deserialize, serde::Serialize, default::Default)]
#[serde(default)]
pub struct Instruction {
    dissassembly: String,
    operandlength: usize,
    #[serde(skip)]
    execute: Option<Instr>,
}
impl Instruction {
    fn new(disassembly: &str, operandlength: usize, execute: Instr) -> Self {
        Self {
            dissassembly: disassembly.to_string(),
            operandlength,
            execute: Some(execute),
        }
    }

    pub fn execute(&self) -> Instr {
        self.execute.unwrap()
    }
}

lazy_static! {
    pub static ref INSTRUCTION_SET: Vec<fn(&mut Chip8)> = {
        vec![cls_or_ret,
        jp,
        call,
        se_vx_byte,
        sne_vx_byte,
        se_vx_vy,
        ld_vx_byte,
        add_vx_byte,
        op_vx_vy,
        skip,
        annn,
        bnnn,
        cxkk,]
        //map.push(Instruction::new("DXYN", 3, dxyn));
        // covers SKP Vx and SKNP Vx
        //map.push(Instruction::new("EX00", 3, dxyn));
        /*
          Seperately calls:
            ld_vx_dt, ld_vx_k, ld_dt_vx, ld_st_vx, add_i_vx, ld_f_vx, ld_b_vx, ld_i_vx. ld_vx_i

         */

    };
}

pub fn cls_or_ret(chip: &mut Chip8) {
    if chip.curr_op & 0xF == 1 {
        cls(chip);
    } else {
        ret(chip);
    }
}

// [00E0] - Clear the Display
// - Requires out put to work.
pub fn cls(_chip: &mut Chip8) {
    todo!("Clear the display")
}

// [00EE] - Return from a subroutine
// Interpreter sets the PC to the address at the top of the stack then subtracts 1 from the stack pointer.
pub fn ret(chip: &mut Chip8) {
    //  Set PC to the top address at the top of the stack
    let top_of_stack = chip.pop_stack();
    chip.program_counter = top_of_stack;
}

// [1NNN] Jump to location NNN
// The interpreter sets the program counter to nnn
pub fn jp(chip: &mut Chip8) {
    let nnn = chip.curr_op & 0x0FFF;
    chip.program_counter = nnn;
}

// [2NNN] Call subroutine at NNN
// Increments stack pointer, puts the current PC on the top of the stack. PC is then set to NNN
pub fn call(chip: &mut Chip8) {
    let nnn = chip.curr_op & 0x0FFF;
    chip.push_stack(chip.program_counter);
    chip.program_counter = nnn;
}

// [3XKK] SE Vx, Byte
// Skip next instruction if Vx ( Register X ) = Byte
pub fn se_vx_byte(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let byte: u8 = chip.curr_op as u8;
    if chip.registers[vx as usize] == byte {
        chip.program_counter += 2;
    }
}

// [4XKK] SNE Vx, Byte
// Skip next instruction if Vx ( Register X ) != Byte
pub fn sne_vx_byte(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let byte: u8 = chip.curr_op as u8;
    if chip.registers[vx as usize] != byte {
        chip.program_counter += 2;
    }
}

// [5XY0] SE Vx, Vy
// Skip next instruction if Vx = Vy (Register_X = Register_Y)
pub fn se_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    if chip.registers[vx as usize] == chip.registers[vy as usize] {
        chip.program_counter += 2;
    }
}

// [6XKK] Load Vx, Byte
// Set Vx = Byte
pub fn ld_vx_byte(chip: &mut Chip8) {
    let _vx = (chip.curr_op >> 8) & 0x0F;
    let _byte = (chip.curr_op & 0xFF) as u8;
    chip.registers[_vx as usize] = _byte;
}

// [7XKK] ADD Vx, Byte
// Set Vx = Vx + Byte
pub fn add_vx_byte(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let byte = chip.curr_op as u8;
    chip.registers[vx as usize] += byte;
}

pub fn op_vx_vy(chip: &mut Chip8) {
    match chip.curr_op & 0xF {
        0x0 => ld_vx_vy(chip),
        0x1 => or_vx_vy(chip),
        0x2 => and_vx_vy(chip),
        0x3 => xor_vx_vy(chip),
        0x4 => add_vx_vy(chip),
        0x5 => sub_vx_vy(chip),
        0x6 => { /*shr_vx_vy(chip)*/ }
        0x7 => { /*subn_vx_vy(chip)*/ }
        0xE => { /*shl_vx_vy  */ }
        _ => unreachable!(),
    }
}

// 8xy0 - LD Vx, Vy
// Set Vx = Vy.
// Stores the value of register Vy in register Vx.
pub fn ld_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    chip.registers[vx as usize] = chip.registers[vy as usize];
}
// 8xy1 - OR Vx, Vy
// Set Vx = Vx OR Vy.
// PERFORMS a bitwise OR on the values of Vx and Vy
// THEN stores the result in Vx.
pub fn or_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    chip.registers[vx as usize] |= chip.registers[vy as usize];
}

// 8xy2 - AND Vx, Vy
// Set Vx = Vx AND Vy.
// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
pub fn and_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    chip.registers[vx as usize] &= chip.registers[vy as usize];
}
// 8xy3 - XOR Vx, Vy
// Set Vx = Vx XOR Vy.
// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
//An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
pub fn xor_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    chip.registers[vx as usize] ^= chip.registers[vy as usize];
}

// 8xy4 - ADD Vx, Vy
// Set Vx = Vx + Vy, set VF = carry.
// The values of Vx and Vy are added together.
// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn add_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;

    let res: u16 = (chip.registers[vx as usize] + chip.registers[vy as usize]) as u16;
    chip.registers[vx as usize] = res as u8;
    chip.registers[0xF_usize] = (res > 255) as u8;
}

// 8xy5 - SUB Vx, Vy
// Set Vx = Vx - Vy, set VF = NOT borrow.
// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
pub fn sub_vx_vy(chip: &mut Chip8) {
    let vx = (chip.curr_op >> 2) & 0x0F;
    let vy = (chip.curr_op >> 1) & 0x00F;
    chip.registers[0xF_usize] = (vx > vy) as u8;
    let res = chip.registers[vx as usize] + chip.registers[vy as usize];
    chip.registers[vx as usize] = res;
}

// 8xy6 - SHR Vx {, Vy}
// Set Vx = Vx SHR 1.
// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.

// 8xy7 - SUBN Vx, Vy
// Set Vx = Vy - Vx, set VF = NOT borrow.
// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

// 8xyE - SHL Vx {, Vy}
// Set Vx = Vx SHL 1.
// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.

// 9xy0 - SNE Vx, Vy
// Skip next instruction if Vx != Vy.
// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
pub fn skip(chip: &mut Chip8) {}
// Annn - LD I, addr
// Set I = nnn.
// The value of register I is set to nnn.
pub fn annn(chip: &mut Chip8) {
    chip.index_register = chip.curr_op & 0x0FFF;
}

// Bnnn - JP V0, addr
// Jump to location nnn + V0
// The program counter is set to nnn plus the value of V0.
pub fn bnnn(chip: &mut Chip8) {
    let _v0 = chip.registers[0] as u16;
    let _nnn= chip.curr_op & (0x0FFF);
    chip.program_counter = _nnn + _v0;
}

// Cxkk - RND Vx, byte
// Set Vx = random byte AND kk.
// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn cxkk(chip: &mut Chip8) {
    let rnd = rand::random::<u8>();
    let kk = chip.curr_op as u8;
    let vx = (chip.curr_op >> 2) & 0x0F;
    chip.registers[vx as usize] = rnd & kk;
}

// Dxyn - DRW Vx, Vy, nibble
// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

// The interpreter reads n bytes from memory, starting at the address stored in I.
// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
// If the sprite is positioned so part of it is outside the coordinates of the display
// it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.

// Ex9E - SKP Vx
// Skip next instruction if key with the value of Vx is pressed.

// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.

// ExA1 - SKNP Vx
// Skip next instruction if key with the value of Vx is not pressed.

// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

// Fx07 - LD Vx, DT
// Set Vx = delay timer value.

// The value of DT is placed into Vx.

// Fx0A - LD Vx, K
// Wait for a key press, store the value of the key in Vx.

// All execution stops until a key is pressed, then the value of that key is stored in Vx.

// Fx15 - LD DT, Vx
// Set delay timer = Vx.

// DT is set equal to the value of Vx.

// Fx18 - LD ST, Vx
// Set sound timer = Vx.

// ST is set equal to the value of Vx.

// Fx1E - ADD I, Vx
// Set I = I + Vx.

// The values of I and Vx are added, and the results are stored in I.

// Fx29 - LD F, Vx
// Set I = location of sprite for digit Vx.

// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

// Fx33 - LD B, Vx
// Store BCD representation of Vx in memory locations I, I+1, and I+2.

// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.

// Fx55 - LD [I], Vx
// Store registers V0 through Vx in memory starting at location I.

// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.

// Fx65 - LD Vx, [I]
// Read registers V0 through Vx from memory starting at location I.

// The interpreter reads values from memory starting at location I into registers V0 through Vx.
