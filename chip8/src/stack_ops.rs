use crate::chip8::Chip8;

impl Chip8 {
    pub fn init_stack(&mut self) {
        self.stack = Default::default();
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.stack.pop_back().unwrap()
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push_back(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Chip8;

    #[test]
    pub fn test_basic_stack_ops() {
        let mut chip8 = Chip8::new();
        chip8.init_stack();
        chip8.push_stack(0x0FFF);
        chip8.push_stack(0xFAF);

        assert_eq!(chip8.pop_stack(), 0xFAF);
        assert_eq!(chip8.pop_stack(), 0xFFF);
    }
}
