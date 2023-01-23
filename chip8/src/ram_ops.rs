use std::{
    fmt::{write, Write},
    path::Path,
    string,
};

use crate::chip8::Chip8;

const TOTAL_RAM_SIZE: usize = 4096;
const FONT_SIZE: usize = 80;
const FREE_RAM_SIZE: usize = TOTAL_RAM_SIZE - FONT_SIZE; // total RAM space - FONT space

const FONT_ADDRESS_START: usize = 0x000;
const FONT_ADDRESS_END: usize = 0x050;
const PROGRAM_ADDRESS_START: usize = 0x200;
const PROGRAM_SPACE_SIZE: usize = TOTAL_RAM_SIZE - 0x200;
impl Chip8 {
    pub fn initialize_ram(&mut self) {
        self.ram = [0; 4096];
        self.load_font();
    }
    pub fn load_font(&mut self) {
        self.ram[0x000..FONT_ADDRESS_END].clone_from_slice(&FONT_ARRAY);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, v) in program.iter().enumerate() {
            self.write_byte(i + PROGRAM_ADDRESS_START, *v)
        }
    }

    // zeroes out the program space (0x200 : 0xFFF)
    pub fn reset_ram(&mut self) {
        self.ram[PROGRAM_ADDRESS_START..].copy_from_slice(&[0; PROGRAM_SPACE_SIZE]);
        self.load_font();
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.ram[address]
    }
    pub fn read_word(&self, address: usize) -> u16 {
        let byte1 = self.ram[address] as u16;
        let byte2 = self.ram[(address + 1)] as u16;
        (byte1 << 8) | byte2
    }

    // Write a byte to ram
    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.ram[address] = byte;
    }

    // Write a word to RAM
    pub fn write_word(_address: usize, word: u16) {
        let _byte1 = (word >> 8) as u8;
        let _byte2 = (word & 0xFF00) as u8;
    }

    pub fn total_ram(&self) -> usize {
        self.ram.len()
    }

    // Ram dump
    pub fn dump_to_file<P: AsRef<Path>>(&self, path: P, bytes_per_row: usize) {
        // println!("{:?}", self.ram.clone())
        use std::fs;
        fs::write(path, self.ram_to_text(bytes_per_row)).expect("Unable to write file");
    }

    pub fn header_to_text(bytes_per_row: usize) -> String {
        let mut string_buf = String::new();
        // generate offset list
        let _ = write!(&mut string_buf, "Offset(h) ");
        for i in 0..bytes_per_row {
            let _ = write!(&mut string_buf, "{:02X} ", i);
        }
        string_buf
    }
    pub fn ram_to_text(&self, bytes_per_row: usize) -> String {
        let mut string_buf = String::new();
        //Write header row

        /*
         * Each row should have an offset that marks where the row starts
         * total_rows = total_len / bytes_per_row (total_length should always be 4096 since that is the amount of memory pre-allocated for storage)
         * If we divide RAM space into 8 bytes  per row, there will be 512 rows
         * Each row will be represented by its offset from 00000000
         * the first row after the 00000000 row following the above example
         * would be  the current row_number where 0 <= row_number < total_rows
         * to calculate the offset for this row.
         * row_offset = row_number * bytes_per_row
         */
        let total_rows = self.total_ram() / bytes_per_row;
        let _ = writeln!(string_buf, "{}\n", Self::header_to_text(bytes_per_row));
        // For each row
        for row_offset in (0..total_rows).map(|row_number| row_number * bytes_per_row) {
            // write row_offset with 2 spaces padding the right to string buffer.
            let _ = write!(string_buf, "{:08X}  ", row_offset);
            // write columns values to string buffer with a space to the right of each value
            for i in 0..bytes_per_row {
                let _ = write!(string_buf, "{:02X} ", self.ram[row_offset + i]);
            }
            let _ = writeln!(string_buf);
        }

        // If total_ram % bytes_per_row != 0
        if self.total_ram() % bytes_per_row != 0 {
            let row_offset = (total_rows) * bytes_per_row;
            let bytes_remaining = self.total_ram() - total_rows * bytes_per_row;

            let _ = write!(string_buf, "{:08X}  ", row_offset);
            for i in 0..bytes_remaining {
                let _ = write!(string_buf, "{:02X} ", self.ram[row_offset + i]);
            }
            let _ = writeln!(string_buf);
        }
        // for row in 0..(self.ram.len() - 8) {
        //     write!(string_buf, "{:08X}  ", row_offset);
        //     writeln!(
        //         string_buf,
        //         "Row {:>4}: {:02X} {:02X} {:02X} {:02X}",
        //         row,
        //         self.ram[row],
        //         self.ram[row + 1],
        //         self.ram[row + 2],
        //         self.ram[row + 3],
        //     )
        //     .expect("Error displaying ram as text");
        // }
        // writeln!(string_buf, "\n").unwrap();
        string_buf
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;
    // use super::*;
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
    pub fn test_reset_ram() {
        let mut chip = Chip8::default();
        chip.initialize_ram();
        chip.load_font();
        chip.load_program(&[0xB2, 0xF2]);
        chip.dump_to_file("../ram_prereset.txt", 8);

        chip.reset_ram();
        chip.dump_to_file("../ram_post_rest.txt", 8);
    }

    #[test]
    pub fn test_dump_to_file() {
        let mut chip8 = Chip8::default();
        chip8.initialize_ram();
        chip8.load_font();
        chip8.dump_to_file("../ram_dump.txt", 8);
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
