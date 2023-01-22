use std::fmt::Write;

use crate::chip8::Chip8;

const FREE_RAM_SIZE: usize = 4096 - 80; // total RAM space - FONT space

impl Chip8 {
    pub fn initialize_ram(&mut self) {
        self.ram = [0; 4096];
        self.load_font();
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, v) in program.iter().enumerate() {
            self.write_byte(i + 512, *v)
        }
    }

    pub fn load_font(&mut self) {
        self.ram[0x000..0x050].clone_from_slice(&FONT_ARRAY);
    }

    pub fn read_byte(&mut self, address: usize) -> u8 {
        self.ram[address]
    }
    pub fn read_word(&mut self, address: usize) -> u16 {
        let byte1 = self.ram[address] as u16;
        let byte2 = self.ram[(address + 1)] as u16;
        (byte1 << 8) | byte2
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.ram[address] = byte;
    }
    pub fn write_word(_address: usize, word: u16) {
        let _byte1 = (word >> 8) as u8;
        let _byte2 = (word & 0xFF00) as u8;
    }

    pub fn reset_ram(&mut self) {
        self.ram[0x200..0xFFF].copy_from_slice(&[0; FREE_RAM_SIZE]);
        self.load_font();
    }

    pub fn dump(&self) {
        println!("{:?}", self.ram.clone())
    }
    pub fn ram_to_text(&mut self) -> String {
        let mut string_buf = String::new();
        for row in 0x1FF..(self.ram.len() - 8) {
            for _col in 0..8 {
                writeln!(
                    string_buf,
                    "0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}",
                    self.ram[row],
                    self.ram[row + 1],
                    self.ram[row + 2],
                    self.ram[row + 3],
                )
                .expect("Error displaying ram as text");
            }
        }
        writeln!(string_buf, "\n").unwrap();
        string_buf
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;
    use super::*;
    #[test]
    pub fn test_font_initialization() {
        let mut chip = Chip8::default();
        chip.load_font();
        assert_eq!(chip.read_byte(0x000), 0xF0);
    }

    #[test]
    pub fn test_byte_rwops() {
        let mut chip = Chip8::default();
        chip.write_byte(0x000, 0xAF);

        assert_eq!(chip.read_byte(0x000), 0xAF);
    }

    #[test]
    pub fn test_ram_to_text() {
        let mut chip8 = Chip8::default();
        chip8.initialize_ram();
        chip8.load_font();
        chip8.load_program(&FONT_ARRAY);
        println!("{}", chip8.ram_to_text())
    }
}

const FONT_ARRAY: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
