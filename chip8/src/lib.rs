pub mod chip8;
mod instruction;
mod ram_ops;
mod stack_ops;
mod test_rom;
mod utils;
pub use chip8::Chip8;

use std::collections::VecDeque;

type Word = u16;
type Byte = u8;
type Tribble = u16;
type Ram = [u8; 4096];
type Stack = VecDeque<u16>;
